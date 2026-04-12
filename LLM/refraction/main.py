"""
Main entry point for the Rust code refactoring tool.
"""

import os
import sys
import glob
import time
import shutil
from datetime import datetime

from .prompts import SYSTEM_PROMPTS, get_prompt_config
from .code_rewriter import rewrite_file, rewrite_file_with_validation, extract_code
from .config import MODEL as DEFAULT_MODEL
from output_manager import OutputManager


def main():
    """Main entry point for the refactoring tool."""
    
    prompt_idx = int(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else 3
    force = "--force" in sys.argv
    validate = "--validate" in sys.argv
    include_negative = "--include-negative" in sys.argv
    negative_only = "--negative-only" in sys.argv
    
    # Parse command line arguments
    tools_str = "compile"
    max_iterations = 5
    model_name = DEFAULT_MODEL
    fallback_strategy = "last-passed"  # Default strategy
    
    for i, arg in enumerate(sys.argv):
        if arg == "--tools" and i + 1 < len(sys.argv):
            tools_str = sys.argv[i + 1]
        elif arg == "--strategy" and i + 1 < len(sys.argv):
            # For backward compatibility, map old strategy names to tools
            tools_str = sys.argv[i + 1]
        elif arg == "--max-iterations" and i + 1 < len(sys.argv):
            max_iterations = int(sys.argv[i + 1])
        elif arg == "--model" and i + 1 < len(sys.argv):
            model_name = sys.argv[i + 1]
        elif arg == "--fallback-strategy" and i + 1 < len(sys.argv):
            fallback_strategy = sys.argv[i + 1]
    
    # Convert tools string to list (support both space and comma separated)
    validation_tools = tools_str.split() if ' ' in tools_str else [tools_str] if tools_str else ["compile"]
    
    # Get prompt configuration
    try:
        prompt_config = get_prompt_config(prompt_idx)
        system_prompt = prompt_config["system_prompt"]
        fixing_prompt = prompt_config["fixing_prompt"]
    except ValueError as e:
        print(f"Error: {e}")
        print(f"Available prompt indices: {list(SYSTEM_PROMPTS.keys())}")
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

    mode_str = f"(validate: {' '.join(validation_tools)}, max_iterations: {max_iterations})" if validate else "(no validation)" if not force else ""
    print(f"🔧 Using SYSTEM_PROMPT_{prompt_idx} {mode_str}")
    print(f"📋 Model: {model_name}")
    print(f"📋 Examples: {sample_type}")
    print(f"📋 Fallback Strategy: {fallback_strategy}\n")
    
    # Initialize OutputManager with timestamp-based directory
    output_manager = OutputManager()
    output_root = output_manager.initialize(
        prompt_idx=prompt_idx,
        validate=validate,
        strategy=tools_str,
        max_iterations=max_iterations,
        force=force,
        model=model_name
    )
    print(f"📁 Output to: {output_root}\n")

    total = len(examples)
    success = 0
    failed = []
    
    start_time = time.time()
    print(f"🔎 Found {total} examples to process ({sample_type})\n")

    for i, filepath in enumerate(examples, 1):
        example_dir = os.path.dirname(filepath)
        example_name = os.path.basename(example_dir)
        
        # Check if this is a negative sample
        is_negative = "/examples_negative/" in filepath
        sample_prefix = "[NEG]" if is_negative else "[POS]"
        
        iter_start_time = time.time()
        print(f"[{i:2d}/{total}] {sample_prefix} 🔄 {example_name} (starting...)")

        # CRITICAL: Ensure main.rs exists for module resolution (pub mod main; in c2rust-lib.rs)
        # Copy main.c2rust.rs to main.rs if it doesn't exist
        main_rs_path = os.path.join(example_dir, "main.rs")
        if not os.path.exists(main_rs_path) and os.path.exists(filepath):
            # filepath points to main.c2rust.rs, so create main.rs from it
            try:
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
                    fixing_prompt,
                    example_dir,
                    max_iterations=max_iterations,
                    validation_tools=validation_tools,
                    output_manager=output_manager,
                    example_name=example_name,
                    model=model_name,
                    fallback_strategy=fallback_strategy
                )
                
                if passed:
                    print(f"       ✅ Passed validation at round {iterations_used}")
                    success += 1
                else:
                    print(f"       ⚠️  Saved (with fallback) - see metadata.json for details")
                    failed.append(example_name)
            else:
                # Original mode (no validation)
                result = rewrite_file(filepath, system_prompt, model=model_name)
                code = extract_code(result)
                
                # Save to timestamped output directory
                output_path = output_manager.save_example_round(example_name, "final", code + "\n")

                print(f"       ✅ Saved")
                success += 1
        except Exception as e:
            print(f"       ❌ Failed: {e}")
            failed.append(example_name)
        
        # Print elapsed time for this example
        iter_elapsed = time.time() - iter_start_time
        remaining = total - i
        if remaining > 0:
            avg_per_example = (time.time() - start_time) / i
            eta_seconds = remaining * avg_per_example
            print(f"       ⏱️  Elapsed: {iter_elapsed:.1f}s | ETA: {eta_seconds:.0f}s for remaining {remaining} examples\n")
        else:
            print(f"       ⏱️  Elapsed: {iter_elapsed:.1f}s\n")

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
    last_output_file = os.path.join(os.path.dirname(__file__), "..", ".last_refactor_output")
    with open(last_output_file, "w") as f:
        f.write(output_manager.output_root)
    print(f"\n✅ Output directory saved to: .last_refactor_output")


if __name__ == "__main__":
    main()
