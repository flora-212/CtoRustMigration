import os
import sys
import glob
import time
import subprocess
import json
import ollama
from validator import CodeValidator, ValidationStrategy, ErrorInfo
from output_manager import OutputManager

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

SYSTEM_PROMPT_3 = """You are a Rust expert. Rewrite the following C2Rust auto-translated Rust code into idiomatic, safe Rust.

Requirements:
1. The code must compile successfully without errors
2. Follow Rust's ownership, borrowing, and concurrency rules
3. Eliminate unsafe blocks as much as possible, especially those related to raw pointers and global mutable state
4. Ensure all data shared across threads is safe:
   - Use Arc<T> for shared ownership
   - Use Mutex<T>, RwLock<T>, or other safe primitives for mutation
5. Do not use non-thread-safe types across threads; replace them with safe abstractions
6. You may restructure the code, including changing function signatures and removing FFI patterns if needed
7. Preserve the program's logical behavior, but not necessarily its exact structure

Output only the complete rewritten code, no explanations
"""

FIXING_PROMPT = """Your previous code failed validation.

{feedback}

Requirements:
1. **Fix all compilation errors first** - these are critical and must be resolved immediately
2. Then address safety and concurrency issues
3. Do not use non-thread-safe types across threads
4. Replace unsafe patterns with safe Rust abstractions

You are allowed to restructure the code as needed:
- Remove extern "C" if not needed
- Remove or redesign *mut c_void arguments
- Replace global static mut with Arc<Mutex<T>>
- Redesign thread interaction patterns
- Change function signatures to be more idiomatic Rust
- Use Arc, Mutex, RwLock, and other standard library synchronization primitives

**Output ONLY the complete corrected code, starting with the first line of code. NO explanations, NO markdown code blocks, just the raw Rust code.**
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

# ────────────────────────────────────────────────────────────────────────────
# Error Formatting Utilities
# ────────────────────────────────────────────────────────────────────────────

def format_errors_for_llm(results: list) -> str:
    """
    Format validation errors for LLM feedback.
    Creates a detailed, structured error report.
    
    Args:
        results: List of ValidationResult objects
    
    Returns:
        Formatted error string for LLM
    """
    parts = []
    
    for result in results:
        if not result.passed:
            parts.append(f"\n[{result.category.upper()}]")
            parts.append(f"Status: FAILED")
            
            # Add structured errors with location info
            if result.errors:
                parts.append(f"\nDetailed errors ({len(result.errors)}):")
                for i, err in enumerate(result.errors[:5], 1):  # Limit to first 5
                    if err.error_code:
                        parts.append(f"  {i}. {err.error_code}: {err.message}")
                    else:
                        parts.append(f"  {i}. {err.error_type}: {err.message}")
                    
                    if err.location:
                        parts.append(f"     Location: {err.location}")
                    
                    if err.line > 0:
                        parts.append(f"     Line {err.line}, Column {err.column}")
            else:
                parts.append(f"Message: {result.message}")
    
    return "\n".join(parts) if parts else "Unknown validation error"


def format_errors_for_display(results: list) -> str:
    """
    Format validation errors for human-readable terminal display.
    Includes visual formatting with symbols and indentation.
    
    Args:
        results: List of ValidationResult objects
    
    Returns:
        Formatted error string for display
    """
    lines = []
    
    for result in results:
        status = "✅" if result.passed else "❌"
        lines.append(f"      {status} [{result.category}] {result.message}")
        
        if not result.passed and result.errors:
            for err in result.errors[:3]:  # Show first 3 errors
                if err.location:
                    lines.append(f"         ├─ {err.error_type}: {err.message}")
                    lines.append(f"         │  📍 {err.location}")
                    if err.error_code:
                        lines.append(f"         └─ Code: {err.error_code}")
                else:
                    lines.append(f"         ├─ {err.message}")
            
            if len(result.errors) > 3:
                lines.append(f"         └─ ... and {len(result.errors) - 3} more error(s)")
    
    return "\n".join(lines)


def extract_errors_for_storage(results: list) -> dict:
    """
    Extract errors in a format suitable for JSON storage.
    
    Args:
        results: List of ValidationResult objects
    
    Returns:
        Dictionary with structured error data
    """
    error_data = {
        "summary": {
            "total_checks": len(results),
            "failed_checks": sum(1 for r in results if not r.passed),
            "total_errors": sum(len(r.errors) for r in results if not r.passed)
        },
        "results": []
    }
    
    for result in results:
        result_dict = {
            "category": result.category,
            "passed": result.passed,
            "message": result.message,
            "errors": [e.to_dict() for e in result.errors]
        }
        error_data["results"].append(result_dict)
    
    return error_data

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
    validation_strategy: ValidationStrategy = ValidationStrategy.COMPILE,
    output_manager: OutputManager = None,
    example_name: str = None
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
        output_manager: OutputManager instance for saving intermediate results
        example_name: Name of the example for organizing outputs
    
    Returns:
        (success: bool, rewritten_code: str, report: str, iterations_used: int)
    """
    # Track failure reasons for all iterations
    iteration_errors = []
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
            
            # Write to file for validation
            output_file = output_manager.save_example_round(
                example_name, 
                iteration, 
                rewritten_code,
            )
            # output_file = output_path + f"round{iteration}.rs"
            # with open(output_file, "w") as f:
            #     f.write(rewritten_code)
            
            # Validate
            passed, report, results = validator.validate_and_report(
                output_file, rewritten_code, validation_strategy, example_dir
            )
            
            if passed:
                print("✅ Validation passed!")
                # # Cleanup output file
                # if os.path.exists(output_file):
                #     os.remove(output_file)
                
                # Save round result if output_manager is provided
                final_path = output_manager.save_example_round(
                    example_name,
                    "final",
                    rewritten_code + "\n"
                )
                
                return True, rewritten_code, report, iteration
                
            else:
                print("❌ Validation failed")
                
                # Show detailed validation feedback to user
                print("\n      📋 Validation Feedback:")
                print(format_errors_for_display(results))
                
                # Extract errors for storage and LLM feedback
                error_data = extract_errors_for_storage(results)
                iteration_error_data = {
                    "iteration": iteration,
                    "passed": False,
                    "error_summary": error_data["summary"],
                    "errors": error_data["results"]
                }
                
                # Add to iteration error history
                iteration_errors.append(iteration_error_data)
                
                # Format feedback for LLM
                llm_feedback = format_errors_for_llm(results)
                
                # Update current code for next iteration
                current_code = rewritten_code

                if iteration < max_iterations:
                    print(f"      🔄 Providing error feedback to LLM (iteration {iteration}/{max_iterations})...\n")
                    
                    messages.append({
                        "role": "user", 
                        "content": FIXING_PROMPT.format(feedback=llm_feedback)
                    })
                    
                    # Rate limit before next attempt
                    time.sleep(RETRY_DELAY)
                else:
                    # Save final (unvalidated) code
                    final_path = output_manager.save_example_round(
                        example_name,
                        "final",
                        rewritten_code + "\n"
                    )
                    
                    # Generate detailed error report
                    error_report_data = {
                        "example_name": example_name,
                        "total_iterations": max_iterations,
                        "failed_at_iteration": max_iterations,
                        "summary": error_data["summary"],
                        "iterations": iteration_errors
                    }
                    output_manager.save_error_report(example_name, error_report_data)
                    print(f"\n      📄 Error details saved for analysis")

        
        except Exception as e:
            print(f"❌ Error: {e}")
            
            # Record exception to error data
            exception_error = {
                "iteration": iteration,
                "passed": False,
                "errors": [{"category": "exception", "message": str(e), "details": ""}],
                "details": {"exception": str(e)}
            }
            iteration_errors.append(exception_error)
            
            if iteration == max_iterations:
                # Generate final error report
                error_report_data = {
                    "example_name": example_name,
                    "total_iterations": max_iterations,
                    "failed_at_iteration": iteration,
                    "iterations": iteration_errors
                }
                if output_manager:
                    output_manager.save_error_report(example_name, error_report_data)
                
                return False, current_code, f"Failed after {max_iterations} iterations: {str(e)}", iteration
            else:
                # Add error to messages for context
                print(f"      🔄 Providing error feedback to LLM: {str(e)[:100]}\n")
                messages.append({"role": "user", "content": f"An error occurred: {str(e)}\n\nPlease try again and fix the code:"})
                time.sleep(RETRY_DELAY)
    
    return False, current_code, f"Failed to pass validation after {max_iterations} iterations", max_iterations

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
    
    # Initialize OutputManager with timestamp-based directory
    output_manager = OutputManager()
    output_root = output_manager.initialize(
        prompt_idx=prompt_idx,
        validate=validate,
        strategy=strategy_str,
        max_iterations=max_iterations,
        force=force
    )
    print(f"📁 Output directory: {output_root}\n")

    examples = sorted(glob.glob("/home/guoxy/concrat/examples/*/main.c2rust.rs"))
    total = len(examples)
    success = 0
    failed = []

    print(f"Found {total} files to process.\n")

    for i, filepath in enumerate(examples, 1):
        example_dir = os.path.dirname(filepath)
        example_name = os.path.basename(example_dir)
        print(f"[{i}/{total}] Processing: {example_name}")

        # if os.path.exists(output_path) and not force:
        #     print(f"  -> Already exists, skipping. (use --force to regenerate)")
        #     success += 1
        #     continue

        try:
            if validate:
                # Iterative validation mode
                print(f"  -> Using iterative validation (max {max_iterations} iterations)...")
                passed, code, report, iterations_used = rewrite_file_with_validation(
                    filepath,
                    system_prompt,
                    example_dir,
                    max_iterations=max_iterations,
                    validation_strategy=validation_strategy,
                    output_manager=output_manager,
                    example_name=example_name
                )
                
                output_path = output_root + f"/examples/{example_name}/final.rs"
                
                # # Save to both old location and new timestamped location
                # with open(output_path, "w") as f:
                #     f.write(code + "\n")
                
                # # Save to timestamped output directory
                # output_manager.save_example_round(example_name, "final", code + "\n")
                
                # Optional: run rustfmt on the output
                subprocess.run(["rustfmt", output_path], capture_output=True)
                
                if passed:
                    print(f"  -> ✅ Saved to: {output_path}")
                    print(f"  -> Iterations used: {iterations_used}/{max_iterations}")
                    print(f"  -> Validation Report:\n{report}")
                    success += 1
                else:
                    print(f"  -> ⚠️  Saved (unvalidated) to: {output_path}")
                    print(f"  -> Iterations used: {iterations_used}/{max_iterations}")
                    print(f"  -> Report: {report}")
                    failed.append(example_name)
            else:
                # Original mode (no validation)
                result = rewrite_file(filepath, system_prompt)
                code = extract_code(result)
                # with open(output_path, "w") as f:
                #     f.write(code + "\n")

                # Save to timestamped output directory
                output_manager.save_example_round(example_name, "final", code + "\n")

                # Optional: run rustfmt on the output
                subprocess.run(["rustfmt", output_path], capture_output=True)

                print(f"  -> Saved to: {output_path}")
                success += 1
        except Exception as e:
            print(f"  -> FAILED: {e}")
            failed.append(example_name)

        # Rate limiting: avoid hitting API too fast
        time.sleep(1)

    # Finalize output manager
    output_manager.finalize(success, total, failed)

    print(f"\n{'='*60}")
    print(f"Results: {success}/{total} succeeded")
    if failed:
        print(f"Failed: {', '.join(failed)}")
    print(f"Output Root: {output_manager.output_root}")
    print(f"Output Dir Structure:")
    print(f"  - config.json")
    print(f"  - rewritten/")
    print(f"  - examples/")
    print(f"  - evaluation/")
    
    # Output timestamp directory info for downstream scripts
    last_output_file = os.path.join(os.path.dirname(__file__), ".last_refactor_output")
    with open(last_output_file, "w") as f:
        f.write(output_manager.output_root)
    print(f"\n✅ Output directory path saved to: {last_output_file}")

if __name__ == "__main__":
    main()