import os
import sys
import glob
import time
import subprocess
import json
from datetime import datetime
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


SYSTEM_PROMPT_4 = """You are a Rust expert. Rewrite the following C2Rust auto-translated Rust code into idiomatic, safe Rust.

Requirements:
1. The code must compile successfully without errors
2. Follow Rust's ownership, borrowing, and concurrency rules
3. Eliminate unsafe blocks as much as possible, especially those related to raw pointers and global mutable state
4. Ensure all data shared across threads is safe:
   - Use Arc<T> for shared ownership
   - Use Mutex<T>, RwLock<T>, or other safe primitives for mutation
5. Do not use non-thread-safe types across threads; replace them with safe abstractions
6. Use capitalized variables and functions for static variables and functions, uncapitalize local variables and functions, following Rust naming conventions
7. You may restructure the code, including changing function signatures and removing FFI patterns if needed
8. Preserve the program's logical behavior, but not necessarily its exact structure
9. Remove Copy and Clone trait implementations if they are not needed, and replace them with appropriate ownership semantics

Output only the complete rewritten code, no explanations
"""

SYSTEM_PROMPT_5 = """You are a Rust expert. Rewrite the following C2Rust auto-translated Rust code into idiomatic, safe Rust.

Requirements:
1 The code MUST compile on stable Rust only. Do NOT use any unstable features or APIs (including core::ffi::c_*); use stable alternatives such as std::ffi or libc.
2. Follow Rust's ownership, borrowing, and concurrency rules
3. Eliminate unsafe blocks as much as possible, especially those related to raw pointers and global mutable state
4. Ensure all data shared across threads is safe:
   - Use Arc<T> for shared ownership
   - Use Mutex<T>, RwLock<T>, or other safe primitives for mutation
5. Do not use non-thread-safe types across threads; replace them with safe abstractions
6. Use capitalized variables and functions for static variables and functions, uncapitalize local variables and functions, following Rust naming conventions
7. In `static` initializations, only use const expressions; Use lazy initialization (e.g., OnceLock or Lazy) if calling non-const functions is necessary. 
8. Do not use libc::NULL; use std::ptr::null() or std::ptr::null_mut() instead
9. Do NOT use pthread_* functions. Replace them with Rust standard concurrency primitives:
   - Use std::thread::spawn instead of pthread_create
   - Use JoinHandle::join instead of pthread_join
   - Use std::sync::Mutex instead of pthread_mutex_t and related lock/unlock calls
   - Ensure all concurrency is implemented using safe Rust abstractions
10. You may restructure the code, including changing function signatures and removing FFI patterns if needed
11. Preserve the program's logical behavior, but not necessarily its exact structure
12. Remove Copy and Clone trait implementations if they are not needed, and replace them with appropriate ownership semantics

Output only the complete rewritten code, no explanations
"""

SYSTEM_PROMPT_6 = """You are a Rust expert. Rewrite the following C2Rust auto-translated Rust code into idiomatic, safe Rust.

Requirements:
1 The code MUST compile on stable Rust only. Do NOT use any unstable features or APIs (including core::ffi::c_*); use stable alternatives such as std::ffi or libc.
2. Follow Rust's ownership, borrowing, and concurrency rules
3. Eliminate unsafe blocks as much as possible, especially those related to raw pointers and global mutable state
4. Ensure all data shared across threads is safe:
   - Use Arc<T> for shared ownership
   - Use Mutex<T>, RwLock<T>, or other safe primitives for mutation
   - If shared mutable access is needed, use Arc<Mutex<T>> or Arc<RwLock<T>>
   - Only call `.lock()` on Mutex<T> or RwLock<T>. Do not call '.lock()' on Arc<Mutex<T>> or Arc<RwLock<T>> directly; instead, clone the Arc and then lock the inner Mutex/RwLock.
5. Use capitalized variables and functions for static variables and functions, uncapitalize local variables and functions, following Rust naming conventions
6. In `static` initializations, only use const expressions; Use lazy initialization (e.g., OnceLock or Lazy) if calling non-const functions is necessary. 
7. Do not use libc::NULL; use std::ptr::null() or std::ptr::null_mut() instead
8. Do NOT use pthread_* functions. Replace them with Rust standard concurrency primitives:
   - Use std::thread::spawn instead of pthread_create
   - Use JoinHandle::join instead of pthread_join
   - Use std::sync::Mutex instead of pthread_mutex_t and related lock/unlock calls
   - Ensure all concurrency is implemented using safe Rust abstractions
9. When initializing arrays:
    - Do NOT use `[expr; N]` if the element type does not implement Copy
    - For non-Copy types (e.g., Mutex, Arc, Vec), initialize each element explicitly or use `std::array::from_fn`
10. You may restructure the code, including changing function signatures and removing FFI patterns if needed
11. Preserve the program's logical behavior, but not necessarily its exact structure
12. Remove Copy and Clone trait implementations if they are not needed, and replace them with appropriate ownership semantics

Output only the complete rewritten code, no explanations
"""


FIXING_PROMPT = """Your previous code failed validation.

{feedback}

Requirements:
1. **Fix all compilation errors first** - these are critical and must be resolved immediately
2. Then address safety and concurrency issues
3. Do NOT use any unstable features or APIs (including core::ffi::c_*); use stable alternatives such as std::ffi or libc.
4. Do not use non-thread-safe types across threads
5. Replace unsafe patterns with safe Rust abstractions
6. You may restructure the code, including changing function signatures and removing FFI patterns if needed
7. Use capitalized variables and functions for static variables and functions, uncapitalize local variables and functions, following Rust naming conventions
8. In `static` initializations, only use const expressions; Use lazy initialization (e.g., OnceLock or Lazy) if calling non-const functions is necessary. 
9. Do not use libc::NULL; use std::ptr::null() or std::ptr::null_mut() instead
10. Do NOT use pthread_* functions. Replace them with Rust standard concurrency primitives:
   - Use std::thread::spawn instead of pthread_create
   - Use JoinHandle::join instead of pthread_join
   - Use std::sync::Mutex instead of pthread_mutex_t and related lock/unlock calls
   - Ensure all concurrency is implemented using safe Rust abstractions

You are allowed to restructure the code as needed:
- Remove or redesign *mut c_void arguments
- Replace global static mut with Arc<Mutex<T>>
- Redesign thread interaction patterns
- Change function signatures to be more idiomatic Rust
- Use Arc, Mutex, RwLock, and other standard library synchronization primitives

**Output ONLY the complete corrected code, starting with the first line of code. NO explanations, NO markdown code blocks, just the raw Rust code.**
"""

# FIXING_PROMPT = """Your previous code failed validation.

# {feedback}

# Requirements:
# 1. **Fix all compilation errors first** - these are critical and must be resolved immediately
# 2. Then address safety and concurrency issues
# 3. Do NOT use any unstable features or APIs (including core::ffi::c_*); use stable alternatives such as std::ffi or libc.
# 4. Follow Rust's ownership, borrowing, and concurrency rules
# 5. Ensure all data shared across threads is safe:
#    - Use Arc<T> for shared ownership
#    - Use Mutex<T>, RwLock<T>, or other safe primitives for mutation
#    - If shared mutable access is needed, use Arc<Mutex<T>> or Arc<RwLock<T>>
#    - Only call `.lock()` on Mutex<T> or RwLock<T>. Do not call '.lock()' on Arc<Mutex<T>> or Arc<RwLock<T>> directly; instead, clone the Arc and then lock the inner Mutex/RwLock.
# 6. Use capitalized variables and functions for static variables and functions, uncapitalize local variables and functions, following Rust naming conventions
# 7. In `static` initializations, only use const expressions; Use lazy initialization (e.g., OnceLock or Lazy) if calling non-const functions is necessary. 
# 8. Do not use libc::NULL; use std::ptr::null() or std::ptr::null_mut() instead
# 9. Do NOT use pthread_* functions. Replace them with Rust standard concurrency primitives:
#    - Use std::thread::spawn instead of pthread_create
#    - Use JoinHandle::join instead of pthread_join
#    - Use std::sync::Mutex instead of pthread_mutex_t and related lock/unlock calls
#    - Ensure all concurrency is implemented using safe Rust abstractions
# 10. When initializing arrays:
#     - Do NOT use `[expr; N]` if the element type does not implement Copy
#     - For non-Copy types (e.g., Mutex, Arc, Vec), initialize each element explicitly or use `std::array::from_fn`
# 11. You may restructure the code, including changing function signatures and removing FFI patterns if needed
# 12. Preserve the program's logical behavior, but not necessarily its exact structure
# 13. Remove Copy and Clone trait implementations if they are not needed, and replace them with appropriate ownership semantics

# You are allowed to restructure the code as needed:
# - Remove or redesign *mut c_void arguments
# - Replace global static mut with Arc<Mutex<T>>
# - Redesign thread interaction patterns
# - Change function signatures to be more idiomatic Rust
# - Use Arc, Mutex, RwLock, and other standard library synchronization primitives

# **Output ONLY the complete corrected code, starting with the first line of code. NO explanations, NO markdown code blocks, just the raw Rust code.**
# """

SYSTEM_PROMPTS = {
    int(k.split("_")[-1]): v
    for k, v in globals().items()
    if k.startswith("SYSTEM_PROMPT_") and isinstance(v, str)

}

# ────────────────────────────────────────────────────────────────────────────
# Parameterized Prompt Configuration
# ────────────────────────────────────────────────────────────────────────────
# This structure allows easy switching between different prompt combinations
# for both system prompts and fixing prompts
# Future extension: can support different prompt sets per prompt_idx if needed

PROMPT_CONFIGS = {
    # prompt_idx: {
    #     "system_prompt": SYSTEM_PROMPT_X,
    #     "fixing_prompt": FIXING_PROMPT (or custom variant)
    # }
    # Currently all indices use the same fixing prompt, but structure allows per-index customization
}

# Initialize PROMPT_CONFIGS with all available system prompts
for idx in SYSTEM_PROMPTS.keys():
    PROMPT_CONFIGS[idx] = {
        "system_prompt": SYSTEM_PROMPTS[idx],
        "fixing_prompt": FIXING_PROMPT,  # Same for all now, but customizable later
    }

def get_prompt_config(prompt_idx: int) -> dict:
    """Get prompt configuration for a given index"""
    if prompt_idx not in PROMPT_CONFIGS:
        raise ValueError(f"Prompt config {prompt_idx} not found. Available: {list(PROMPT_CONFIGS.keys())}")
    return PROMPT_CONFIGS[prompt_idx]

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
    fixing_prompt: str,
    example_dir: str,
    max_iterations: int = 5,
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
    
    # Track compilation status for all rounds
    rounds_compile_status = {}  # {round_num: compile_status}
    best_compile_round = None    # Best round that compiled successfully
    best_compile_code = None     # Code that compiled best
    
    with open(filepath, "r") as f:
        original_code = f.read()
    
    current_code = original_code
    messages = [{"role": "system", "content": system_prompt}]
    
    for iteration in range(1, max_iterations + 1):
        print(f"       [R{iteration:d}/{max_iterations}]", end=" ", flush=True)
        
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
            
            # Validate the generated file
            # CRITICAL: Pass the actual generated file path (output_file) not the original
            passed, report, results = validator.validate_and_report(
                output_file, rewritten_code, validation_strategy, example_dir
            )
            
            # Track compilation status - check if compiler passed (only check "compilation" category)
            compilation_results = [r for r in results if r.category == "compilation"]
            compile_passed = all(r.passed for r in compilation_results) if compilation_results else passed
            rounds_compile_status[iteration] = compile_passed
            
            # Update best compile round if this one compiled
            if compile_passed and best_compile_round is None:
                best_compile_round = iteration
                best_compile_code = rewritten_code
            elif compile_passed:
                # Update to latest compile-passing version
                best_compile_round = iteration
                best_compile_code = rewritten_code
            
            # Extract errors for storage and LLM feedback
            error_data = extract_errors_for_storage(results)
            
            # Save round metadata with error info
            if output_manager:
                output_manager.save_round_metadata(example_name, iteration, passed, compile_passed, error_data)
            
            if passed:
                print("✅ Passed!", end=" ")
                # # Cleanup output file
                # if os.path.exists(output_file):
                #     os.remove(output_file)
                
                # Save round result if output_manager is provided
                final_path = output_manager.save_example_round(
                    example_name,
                    "final",
                    rewritten_code + "\n"
                )
                
                # Save conversation history
                if output_manager:
                    output_manager.save_conversation_history(example_name, messages)
                
                return True, rewritten_code, report, iteration
                
            else:
                compile_status = "✅" if compile_passed else "❌"
                print(f"Failed ({compile_status} compile)")
                
                # Print detailed error information
                error_summary = error_data["summary"]
                print(f"  Failed checks: {error_summary['failed_checks']}/{error_summary['total_checks']}")
                print(f"  Total errors: {error_summary['total_errors']}")
                
                # Print errors by category
                for error_result in error_data["results"]:
                    if not error_result["passed"]:
                        print(f"  ❌ {error_result['category']}: {error_result['message']}")
                        for error in error_result["errors"]:
                            error_msg = error.get("message", error.get("error", str(error)))
                            # Truncate long error messages
                            if len(error_msg) > 100:
                                error_msg = error_msg[:97] + "..."
                            print(f"     • {error_msg}")
                
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
                    print(f" 🔄 trying again...")
                    
                    messages.append({
                        "role": "user", 
                        "content": fixing_prompt.format(feedback=llm_feedback)
                    })
                    
                    # Rate limit before next attempt
                    time.sleep(RETRY_DELAY)
                else:
                    # All iterations done but still not validated
                    # Try to use best compile-passing version
                    final_code = None
                    final_round = None
                    final_reason = None
                    
                    print()  # newline
                    if best_compile_round is not None and best_compile_code is not None:
                        print(f"       💾 Using best compile from round {best_compile_round}")
                        final_code = best_compile_code
                        final_round = best_compile_round
                        final_reason = "best_compile_round"
                    else:
                        # No LLM-generated version compiled - try original code as fallback
                        print(f"       🔄 Trying original unmodified code...")
                        
                        # Try to compile original code
                        original_compile_ok = False
                        try:
                            # Write original code to temp file and check
                            temp_file = os.path.join(
                                output_manager.get_example_dir(example_name) if output_manager else "/tmp",
                                "_original_check.rs"
                            )
                            with open(temp_file, "w") as f:
                                f.write(original_code)
                            
                            original_compile_ok, _ = validator.try_compile_standalone(temp_file, example_dir)
                            
                            if os.path.exists(temp_file):
                                os.remove(temp_file)
                        except:
                            original_compile_ok = False
                        
                        if original_compile_ok:
                            print(f"       ✅ Original code compiles! Using it.")
                            final_code = original_code
                            final_round = "original"
                            final_reason = "original_code_fallback"
                        else:
                            # Last resort - save last attempted LLM version
                            print(f"       ⚠️  Original also fails. Saving last LLM attempt.")
                            final_code = rewritten_code
                            final_round = max_iterations
                            final_reason = "last_attempt_no_compile"
                    
                    # Save final version
                    if final_code:
                        final_path = output_manager.save_example_round(
                            example_name,
                            "final",
                            final_code + "\n"
                        )
                        # Save metadata indicating which source was used
                        final_metadata = {
                            "result_type": final_reason,
                            "round": final_round,
                            "reason": final_reason,
                            "timestamp": datetime.now().isoformat()
                        }
                        final_meta_path = os.path.join(
                            output_manager.get_example_dir(example_name),
                            "final_metadata.json"
                        )
                        with open(final_meta_path, "w") as f:
                            json.dump(final_metadata, f, indent=2, ensure_ascii=False)
                    
                    # Generate detailed error report
                    error_report_data = {
                        "example_name": example_name,
                        "total_iterations": max_iterations,
                        "failed_at_iteration": max_iterations,
                        "best_compile_round": best_compile_round,
                        "final_source": final_round,
                        "summary": error_data["summary"],
                        "iterations": iteration_errors,
                        "rounds_compile_status": rounds_compile_status
                    }
                    output_manager.save_error_report(example_name, error_report_data)
                    print(f"       📄 Details saved")
                    
                    # Save conversation history
                    if output_manager:
                        output_manager.save_conversation_history(example_name, messages)

        
        except Exception as e:
            print(f"Exception: {str(e)[:50]}")
            
            # Record exception to error data
            exception_error = {
                "iteration": iteration,
                "passed": False,
                "errors": [{"category": "exception", "message": str(e), "details": ""}],
                "details": {"exception": str(e)}
            }
            iteration_errors.append(exception_error)
            
            if iteration == max_iterations:
                # Last iteration with exception
                # Try fallback: best compile round or original code
                final_code = None
                final_round = None
                final_reason = None
                
                print()  # newline
                if best_compile_round is not None and best_compile_code is not None:
                    print(f"       💾 Using best compile from round {best_compile_round}")
                    final_code = best_compile_code
                    final_round = best_compile_round
                    final_reason = "best_compile_round"
                else:
                    # Try original code fallback
                    print(f"       🔄 Trying original unmodified code...")
                    original_compile_ok = False
                    try:
                        temp_file = os.path.join(
                            output_manager.get_example_dir(example_name) if output_manager else "/tmp",
                            "_original_check.rs"
                        )
                        with open(temp_file, "w") as f:
                            f.write(original_code)
                        original_compile_ok, _ = validator.try_compile_standalone(temp_file, example_dir)
                        if os.path.exists(temp_file):
                            os.remove(temp_file)
                    except:
                        original_compile_ok = False
                    
                    if original_compile_ok:
                        print(f"       ✅ Original code compiles! Using it.")
                        final_code = original_code
                        final_round = "original"
                        final_reason = "original_code_fallback"
                    else:
                        print(f"       ⚠️  Saving last LLM attempt")
                        final_code = best_compile_code or current_code
                        final_round = best_compile_round or max_iterations
                        final_reason = "last_attempt_exception"
                
                if final_code and output_manager:
                    output_manager.save_example_round(example_name, "final", final_code + "\n")
                    final_metadata = {
                        "result_type": final_reason,
                        "round": final_round,
                        "reason": final_reason,
                        "timestamp": datetime.now().isoformat()
                    }
                    final_meta_path = os.path.join(
                        output_manager.get_example_dir(example_name),
                        "final_metadata.json"
                    )
                    with open(final_meta_path, "w") as f:
                        json.dump(final_metadata, f, indent=2, ensure_ascii=False)
                
                error_report_data = {
                    "example_name": example_name,
                    "total_iterations": max_iterations,
                    "failed_at_iteration": iteration,
                    "best_compile_round": best_compile_round,
                    "final_source": final_round,
                    "iterations": iteration_errors,
                    "rounds_compile_status": rounds_compile_status
                }
                if output_manager:
                    output_manager.save_error_report(example_name, error_report_data)
                    output_manager.save_conversation_history(example_name, messages)
                
                return False, final_code or current_code, f"Failed after {max_iterations} iterations: {str(e)}", iteration
            else:
                # Add error to messages for context
                print(f" 🔄 retrying...")
                messages.append({"role": "user", "content": f"An error occurred: {str(e)}\n\nPlease try again and fix the code:"})
                time.sleep(RETRY_DELAY)
    
    # Final return if all iterations exhausted without passing
    final_code = None
    final_round = None
    final_reason = None
    
    if best_compile_round is not None and best_compile_code is not None:
        final_code = best_compile_code
        final_round = best_compile_round
        final_reason = "best_compile_round"
    else:
        # Try original code fallback
        original_compile_ok = False
        try:
            temp_file = os.path.join(
                output_manager.get_example_dir(example_name) if output_manager else "/tmp",
                "_original_check.rs"
            )
            with open(temp_file, "w") as f:
                f.write(original_code)
            original_compile_ok, _ = validator.try_compile_standalone(temp_file, example_dir)
            if os.path.exists(temp_file):
                os.remove(temp_file)
        except:
            original_compile_ok = False
        
        if original_compile_ok:
            final_code = original_code
            final_round = "original"
            final_reason = "original_code_fallback"
        else:
            final_code = current_code
            final_round = max_iterations
            final_reason = "last_attempt"
    
    if final_code and output_manager:
        output_manager.save_example_round(example_name, "final", final_code + "\n")
        final_metadata = {
            "result_type": final_reason,
            "round": final_round,
            "reason": final_reason,
            "timestamp": datetime.now().isoformat()
        }
        final_meta_path = os.path.join(
            output_manager.get_example_dir(example_name),
            "final_metadata.json"
        )
        with open(final_meta_path, "w") as f:
            json.dump(final_metadata, f, indent=2, ensure_ascii=False)
    
    # Save conversation history
    if output_manager:
        output_manager.save_conversation_history(example_name, messages)
    
    return False, final_code or current_code, f"Failed to pass validation after {max_iterations} iterations", max_iterations

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
    prompt_idx = int(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else 3
    force = "--force" in sys.argv
    validate = "--validate" in sys.argv
    include_negative = "--include-negative" in sys.argv
    negative_only = "--negative-only" in sys.argv
    
    # Parse command line arguments
    strategy_str = "compile"
    max_iterations = 5
    
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
    
    # Get prompt configuration
    try:
        prompt_config = get_prompt_config(prompt_idx)
        system_prompt = prompt_config["system_prompt"]
        fixing_prompt = prompt_config["fixing_prompt"]
    except ValueError as e:
        print(f"Error: {e}")
        print(f"Available prompt indices: {list(PROMPT_CONFIGS.keys())}")
        sys.exit(1)

    # Determine data sources
    if negative_only:
        sample_type = "negative only"
        examples = sorted(glob.glob("/home/guoxy/concrat/examples_negative/*/main.c2rust.rs"))
    elif include_negative:
        sample_type = "positive + negative"
        positive = sorted(glob.glob("/home/guoxy/concrat/examples/*/main.c2rust.rs"))
        negative = sorted(glob.glob("/home/guoxy/concrat/examples_negative/*/main.c2rust.rs"))
        examples = positive + negative
    else:
        sample_type = "positive only"
        examples = sorted(glob.glob("/home/guoxy/concrat/examples/*/main.c2rust.rs"))

    mode_str = f"(validate: {validation_strategy.value}, max_iterations: {max_iterations})" if validate else "(no validation)" if not force else ""
    print(f"🔧 Using SYSTEM_PROMPT_{prompt_idx} {mode_str}")
    print(f"📋 Examples: {sample_type}")
    print(f"📋 Strategy: Save best compilable version + original fallback\n")
    
    # Initialize OutputManager with timestamp-based directory
    output_manager = OutputManager()
    output_root = output_manager.initialize(
        prompt_idx=prompt_idx,
        validate=validate,
        strategy=strategy_str,
        max_iterations=max_iterations,
        force=force
    )
    print(f"📁 Output to: {output_root}\n")

    total = len(examples)
    success = 0
    failed = []

    print(f"🔎 Found {total} examples to process ({sample_type})\n")

    for i, filepath in enumerate(examples, 1):
        example_dir = os.path.dirname(filepath)
        example_name = os.path.basename(example_dir)
        
        # Check if this is a negative sample
        is_negative = "/examples_negative/" in filepath
        sample_prefix = "[NEG]" if is_negative else "[POS]"
        
        print(f"[{i:2d}/{total}] {sample_prefix} 🔄 {example_name}")

        # CRITICAL: Ensure main.rs exists for module resolution (pub mod main; in c2rust-lib.rs)
        # Copy main.c2rust.rs to main.rs if it doesn't exist
        main_rs_path = os.path.join(example_dir, "main.rs")
        if not os.path.exists(main_rs_path) and os.path.exists(filepath):
            # filepath points to main.c2rust.rs, so create main.rs from it
            try:
                import shutil
                shutil.copy(filepath, main_rs_path)
                print(f"       ℹ️  Created main.rs")
            except Exception as e:
                print(f"       ⚠️  Could not create main.rs: {e}")

        try:
            if validate:
                # Iterative validation mode
                print(f"       🔄 Iterative validation (max {max_iterations} rounds)")
                passed, code, report, iterations_used = rewrite_file_with_validation(
                    filepath,
                    system_prompt,
                    fixing_prompt,  # Pass the fixing_prompt here
                    example_dir,
                    max_iterations=max_iterations,
                    validation_strategy=validation_strategy,
                    output_manager=output_manager,
                    example_name=example_name
                )
                
                if passed:
                    print(f"       ✅ Passed validation at round {iterations_used}")
                    success += 1
                else:
                    print(f"       ⚠️  Saved (with fallback) - see metadata.json for details")
                    failed.append(example_name)
            else:
                # Original mode (no validation)
                result = rewrite_file(filepath, system_prompt)
                code = extract_code(result)
                
                # Save to timestamped output directory
                output_path = output_manager.save_example_round(example_name, "final", code + "\n")

                print(f"       ✅ Saved")
                success += 1
        except Exception as e:
            print(f"       ❌ Failed: {e}")
            failed.append(example_name)

        # Rate limiting: avoid hitting API too fast
        time.sleep(1)

    # Finalize output manager
    output_manager.finalize(success, total, failed)

    print(f"\n{'='*70}")
    print(f"✅ Results: {success}/{total} examples processed")
    if failed:
        print(f"⚠️  Failed to pass validation: {', '.join(failed[:5])}" + (f" + {len(failed)-5} more..." if len(failed) > 5 else ""))
    
    print(f"\n📁 Output Structure:")
    print(f"   {output_manager.output_root}/")
    print(f"   ├── examples/")
    print(f"   │   ├── {{example_name}}/")
    print(f"   │   │   ├── round1.rs, round2.rs, ...   # Iteration history")
    print(f"   │   │   ├── final.rs                     # Final version (best/fallback)")
    print(f"   │   │   ├── rounds_metadata.json         # All rounds metadata")
    print(f"   │   │   ├── final_metadata.json          # Which source was used")
    print(f"   │   │   ├── conversation_history.json    # Full LLM conversation")
    print(f"   │   │   └── error_report.json")
    print(f"   │   └── ...")
    print(f"   ├── config.json")
    print(f"   └── evaluation/")
    
    print(f"\n📊 Summary:")
    print(f"   - Sample type: {sample_type}")
    print(f"   - Total processed: {total}")
    print(f"   - Validation passed: {success}")
    print(f"   - Using fallback/best: {total - success}")
    
    # Output timestamp directory info for downstream scripts
    last_output_file = os.path.join(os.path.dirname(__file__), ".last_refactor_output")
    with open(last_output_file, "w") as f:
        f.write(output_manager.output_root)
    print(f"\n✅ Output directory saved to: .last_refactor_output")

if __name__ == "__main__":
    main()