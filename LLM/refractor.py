import os
import sys
import glob
import time
import subprocess
import ollama
from validator import CodeValidator, ValidationStrategy

# ────────────────────────────────────────────────────────────────────────────
# System Prompts for different optimization levels
# ────────────────────────────────────────────────────────────────────────────

SYSTEM_PROMPT_0 = """You are a Rust expert. Rewrite the following C2Rust auto-translated Rust code into idiomatic, safe Rust.
Requirements:
1. Make sure the code compiles without errors
2. Eliminate unsafe blocks as much as possible, especially those related to raw pointers and manual memory management
5. Keep functionality exactly the same
6. Output only the complete rewritten code, no explanations
"""

SYSTEM_PROMPT_1 = """You are a Rust expert. Rewrite the following C2Rust auto-translated Rust code into idiomatic, safe Rust.
Requirements:
1. Make sure the code compiles without errors
2. Replace pthread_mutex with std::sync::Mutex
3. Replace pthread_create/pthread_join with std::thread
4. Remove unnecessary libc types, use native Rust types
5. Keep functionality exactly the same. Output only the complete rewritten code, no explanations
"""

SYSTEM_PROMPT_2 = """You are a Rust expert. Rewrite the following C2Rust auto-translated Rust code into idiomatic, safe Rust.
Requirements:
1. Ensure the rewritten code compiles successfully without errors.
2. Replace pthread-based synchronization primitives with Rust standard library equivalents:
   - Replace pthread_mutex with std::sync::Mutex
   - Replace pthread_rwlock with std::sync::RwLock
   - Replace spin locks with std::sync::Mutex or std::sync::RwLock
   - Replace pthread condition variables with std::sync::Condvar
3. Replace pthread_create and pthread_join with std::thread.
4. Remove unnecessary libc types and use native Rust types whenever possible.
5. Prefer safe Rust abstractions and minimize or eliminate unsafe blocks if possible.
6. Keep the program's functionality and concurrency semantics exactly the same.
"""


SYSTEM_PROMPTS = {
    int(k.split("_")[-1]): v
    for k, v in globals().items()
    if k.startswith("SYSTEM_PROMPT_") and isinstance(v, str)

}

# ────────────────────────────────────────────────────────────────────────────
# Configuration
# ────────────────────────────────────────────────────────────────────────────

MODEL = "qwen2.5-coder:14b"
MAX_RETRIES = 3
RETRY_DELAY = 5  # seconds
VALIDATION_STRATEGY = ValidationStrategy.COMPILE  # Default validation strategy

def rewrite_file(filepath: str, system_prompt: str) -> str:
    with open(filepath, "r") as f:
        code = f.read()

    for attempt in range(MAX_RETRIES):
        try:
            response = ollama.chat(
                model=MODEL,
                messages=[
                    {"role": "system", "content": system_prompt},
                    {"role": "user", "content": f"Rewrite the following code:\n\n```rust\n{code}\n```"},
                ],
                options={"num_ctx": 8192, "temperature": 0.2},
            )
            return response.message.content
        except Exception as e:
            print(f"  Attempt {attempt + 1}/{MAX_RETRIES} failed: {e}")
            if attempt < MAX_RETRIES - 1:
                time.sleep(RETRY_DELAY)
            else:
                raise

def rewrite_file_with_validation(
    filepath: str,
    system_prompt: str,
    example_dir: str,
    max_iterations: int = 3,
    validation_strategy: ValidationStrategy = ValidationStrategy.COMPILE
) -> tuple:
    """
    Rewrite a Rust file with iterative validation and feedback.
    
    If compilation/validation fails, the error is fed back to the LLM
    for another attempt. This creates an iterative improvement loop.
    
    Args:
        filepath: Original .rs file to rewrite
        system_prompt: System prompt for LLM
        example_dir: Directory containing Cargo.toml (for cargo build context)
        max_iterations: Maximum number of generation attempts
        validation_strategy: What to validate (compile, safety, lock_safety, comprehensive)
    
    Returns:
        (success: bool, rewritten_code: str, report: str)
    """
    validator = CodeValidator()
    
    with open(filepath, "r") as f:
        original_code = f.read()
    
    current_code = original_code
    messages = [{"role": "system", "content": system_prompt}]
    
    for iteration in range(1, max_iterations + 1):
        print(f"    [Iteration {iteration}/{max_iterations}]", end=" ", flush=True)
        
        try:
            # Request rewrite/fix
            user_msg = f"Rewrite the following code:\n\n```rust\n{current_code}\n```"
            messages.append({"role": "user", "content": user_msg})
            
            response = ollama.chat(
                model=MODEL,
                messages=messages,
                options={"num_ctx": 8192, "temperature": 0.2},
            )
            
            rewritten_code = extract_code(response.message.content)
            messages.append({"role": "assistant", "content": response.message.content})
            
            # Write to temp file for validation
            temp_file = filepath.replace(".rs", f"_temp_{iteration}.rs")
            with open(temp_file, "w") as f:
                f.write(rewritten_code)
            
            # Validate
            passed, report, results = validator.validate_and_report(
                temp_file, rewritten_code, validation_strategy, example_dir
            )
            
            if passed:
                print("✅ Validation passed!")
                # Cleanup temp file
                if os.path.exists(temp_file):
                    os.remove(temp_file)
                return True, rewritten_code, report
            else:
                print("❌ Validation failed")
                
                # 显示详细的验证反馈
                print("\n      📋 验证反馈详情:")
                for result in results:
                    status = "✅" if result.passed else "❌"
                    print(f"      {status} [{result.category}] {result.message}")
                    
                    # 如果是编译错误，显示详细的错误信息
                    if not result.passed and result.details.get("error"):
                        error_lines = result.details["error"].split("\n")[:3]  # 显示前 3 行
                        for err_line in error_lines:
                            if err_line.strip():
                                print(f"         └─ {err_line[:100]}")
                
                # Prepare feedback for next iteration
                feedback_lines = []
                for result in results:
                    if not result.passed:
                        # 提取更详细的错误信息用于反馈
                        if result.category == "compile" and result.details.get("error"):
                            # 只取编译错误的关键信息
                            error_summary = result.details["error"].split("\n")[0:2]
                            error_text = " | ".join(error_summary)
                            feedback_lines.append(f"[{result.category}] {error_text[:150]}")
                        else:
                            feedback_lines.append(f"[{result.category}] {result.message}")
                
                feedback = "Validation issues found:\n" + "\n".join(feedback_lines)
                
                # 如果不是最后一次迭代，向 LLM 反馈
                if iteration < max_iterations:
                    print(f"      🔄 向 LLM 反馈错误信息...\n")
                    # 直接向消息中添加反馈
                    messages.append({"role": "user", "content": f"Please fix these issues:\n{feedback}\n\nRewrite the code again:"})
                
                # Update current code for next iteration
                current_code = rewritten_code
                
                # Cleanup temp file
                if os.path.exists(temp_file):
                    os.remove(temp_file)
        
        except Exception as e:
            print(f"❌ Error: {e}")
            if iteration == max_iterations:
                return False, current_code, f"Failed after {max_iterations} iterations: {str(e)}"
            else:
                # Add error to messages for context
                print(f"      🔄 向 LLM 反馈错误: {str(e)[:100]}\n")
                messages.append({"role": "user", "content": f"An error occurred: {str(e)}\n\nPlease try again and fix the code:"})
                time.sleep(RETRY_DELAY)
    
    return False, current_code, f"Failed to pass validation after {max_iterations} iterations"

def extract_code(result: str) -> str:
    """Extract code from markdown code block."""
    if "```rust" in result:
        result = result.split("```rust", 1)[1].split("```", 1)[0]
    elif "```" in result:
        result = result.split("```", 1)[1].split("```", 1)[0]
    return result.strip()

def verify_syntax(filepath: str) -> bool:
    """Check if the rewritten file has valid Rust syntax."""
    result = subprocess.run(
        ["rustfmt", "--check", filepath],
        capture_output=True, text=True
    )
    return result.returncode == 0

def main():
    prompt_idx = int(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else 0
    force = "--force" in sys.argv
    validate = "--validate" in sys.argv
    
    # Parse command line arguments
    strategy_str = "compile"
    max_iterations = 3
    
    for i, arg in enumerate(sys.argv):
        if arg == "--strategy" and i + 1 < len(sys.argv):
            strategy_str = sys.argv[i + 1]
        elif arg == "--max-iterations" and i + 1 < len(sys.argv):
            max_iterations = int(sys.argv[i + 1])
    
    try:
        validation_strategy = ValidationStrategy(strategy_str)
    except ValueError:
        print(f"Error: Unknown strategy '{strategy_str}'")
        print(f"Available: {', '.join([s.value for s in ValidationStrategy])}")
        sys.exit(1)
    
    if prompt_idx not in SYSTEM_PROMPTS:
        print(f"Error: SYSTEM_PROMPT_{prompt_idx} does not exist. Available: {list(SYSTEM_PROMPTS.keys())}")
        sys.exit(1)

    system_prompt = SYSTEM_PROMPTS[prompt_idx]
    mode_str = f"(validate: {validation_strategy.value}, max_iterations: {max_iterations})" if validate else "(force regenerate)" if force else ""
    print(f"Using SYSTEM_PROMPT_{prompt_idx} {mode_str}\n")

    examples = sorted(glob.glob("/home/guoxy/concrat/examples/*/main.rs"))
    total = len(examples)
    success = 0
    failed = []

    print(f"Found {total} files to process.\n")

    for i, filepath in enumerate(examples, 1):
        example_dir = os.path.dirname(filepath)
        example_name = os.path.basename(example_dir)
        print(f"[{i}/{total}] Processing: {example_name}")

        output_path = filepath.replace("main.rs", f"main_rewritten_{prompt_idx}.rs")

        if os.path.exists(output_path) and not force:
            print(f"  -> Already exists, skipping. (use --force to regenerate)")
            success += 1
            continue

        try:
            if validate:
                # Iterative validation mode
                print(f"  -> Using iterative validation (max {max_iterations} iterations)...")
                passed, code, report = rewrite_file_with_validation(
                    filepath,
                    system_prompt,
                    example_dir,
                    max_iterations=max_iterations,
                    validation_strategy=validation_strategy
                )
                
                with open(output_path, "w") as f:
                    f.write(code + "\n")
                
                # Optional: run rustfmt on the output
                subprocess.run(["rustfmt", output_path], capture_output=True)
                
                if passed:
                    print(f"  -> ✅ Saved to: {output_path}")
                    print(f"  -> Validation Report:\n{report}")
                    success += 1
                else:
                    print(f"  -> ⚠️  Saved (unvalidated) to: {output_path}")
                    print(f"  -> Report: {report}")
                    failed.append(example_name)
            else:
                # Original mode (no validation)
                result = rewrite_file(filepath, system_prompt)
                code = extract_code(result)
                with open(output_path, "w") as f:
                    f.write(code + "\n")

                # Optional: run rustfmt on the output
                subprocess.run(["rustfmt", output_path], capture_output=True)

                print(f"  -> Saved to: {output_path}")
                success += 1
        except Exception as e:
            print(f"  -> FAILED: {e}")
            failed.append(example_name)

        # Rate limiting: avoid hitting API too fast
        time.sleep(1)

    print(f"\n{'='*60}")
    print(f"Results: {success}/{total} succeeded")
    if failed:
        print(f"Failed: {', '.join(failed)}")
    print(f"Output: {prompt_idx}")

if __name__ == "__main__":
    main()