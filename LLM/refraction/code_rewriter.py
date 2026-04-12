"""
Code rewriting functionality using LLM for Rust code refactoring.
"""

import os
import json
import time
from datetime import datetime
from typing import Optional, List
import ollama

from .config import MAX_RETRIES, RETRY_DELAY
from .prompts import SYSTEM_PROMPTS
from .error_formatter import format_errors_for_llm, extract_errors_for_storage
from .utils import extract_code
from validation import CodeValidator


def rewrite_file(filepath: str, system_prompt: str, model: str = "qwen2.5-coder:14b") -> str:
    """
    Simple rewrite of a file without validation.
    
    Args:
        filepath: Path to the .rs file to rewrite
        system_prompt: System prompt for the LLM
        model: Model name to use
    
    Returns:
        The rewritten code
    """
    with open(filepath, "r") as f:
        code = f.read()

    for attempt in range(MAX_RETRIES):
        try:
            response = ollama.chat(
                model=model,
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
    validation_tools: Optional[List[str]] = None,
    output_manager = None,
    example_name: str = None,
    model: str = "qwen2.5-coder:14b",
    fallback_strategy: str = "last-passed"
) -> tuple:
    """
    Rewrite a Rust file with iterative validation and feedback.
    
    If compilation/validation fails, the error is fed back to the LLM
    for another attempt. This creates an iterative improvement loop.
    
    Args:
        filepath: Original .rs file to rewrite
        system_prompt: System prompt for LLM
        fixing_prompt: Prompt for fixing failed validations
        example_dir: Directory containing Cargo.toml (for cargo build context)
        max_iterations: Maximum number of generation attempts
        validation_tools: Tools to validate with (e.g., ['compile', 'clippy', 'safety'])
        output_manager: OutputManager instance for saving intermediate results
        example_name: Name of the example for organizing outputs
        model: Model name to use
        fallback_strategy: Strategy when max_iterations reached without passing:
            - 'last-passed' (default): Save the last round that compiled
            - 'last-round': Save the final round regardless of compilation status
    
    Returns:
        (success: bool, rewritten_code: str, report: str, iterations_used: int)
    """
    # Provide default validation tools
    if validation_tools is None:
        validation_tools = ["compile"]
    
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
                model=model,
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
            validation_msg = "Validating"
            if any(tool in validation_tools for tool in ["miri"]):
                validation_msg = "Validating (⏳ miri runs slowly, please be patient)"
            print(f"           {validation_msg}... [1/{max_iterations}]", end="", flush=True)
            
            passed, report, results = validator.validate_and_report(
                output_file, rewritten_code, validation_tools, example_dir
            )
            
            print(f"\r           Validated [1/{max_iterations}]")  # Clear the loading message
            
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
                    final_code = None
                    final_round = None
                    final_reason = None
                    
                    print()  # newline
                    
                    # Determine strategy
                    if fallback_strategy == "last-round":
                        # Strategy: Always save the final round regardless of compilation
                        print(f"       💾 Using last round (round {max_iterations}) - fallback strategy: last-round")
                        final_code = rewritten_code
                        final_round = max_iterations
                        final_reason = "last_round_fallback"
                    else:
                        # Strategy: last-passed (default) - find best compiled version
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
