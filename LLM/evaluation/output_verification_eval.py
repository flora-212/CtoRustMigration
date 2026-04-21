#!/usr/bin/env python3
"""
Output verification for LLM-generated Rust code.

Verifies that generated code produces correct output.
Reads final.rs files, appends test module for output verification, and runs cargo test.
Generates readable markdown table report.
"""

import os
import sys
import json
import glob
import shutil
import tempfile
import subprocess
import argparse
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional
import time

# Paths
EXAMPLES_DIR = "/home/guoxy/concrat/examples"
# EXAMPLES_NEGATIVE_DIR = "/home/guoxy/concrat/examples_negative"
TEST_TIMEOUT = 300  # 5 minutes per test


class OutputTestResult:
    """Represents result of output verification testing one example."""
    
    def __init__(self, example_name: str, success: bool, message: str = "", details: str = "", time_sec: float = 0.0):
        self.example_name = example_name
        self.success = success
        self.message = message
        self.details = details
        self.time_sec = time_sec
    
    def to_dict(self) -> dict:
        return {
            "example_name": self.example_name,
            "success": self.success,
            "message": self.message,
            "time_sec": self.time_sec,
            "details": self.details[:500] if self.details else ""
        }


def find_example_dir(example_name: str) -> Optional[str]:
    """Find original example directory (positive or negative)."""
    positive = os.path.join(EXAMPLES_DIR, example_name)
    if os.path.exists(positive):
        return positive
    
    # negative = os.path.join(EXAMPLES_NEGATIVE_DIR, example_name)
    # if os.path.exists(negative):
    #     return negative
    
    return None


def setup_output_test_dir(
    final_rs: str,
    example_dir: str,
    example_name: str,
    test_dir: str
) -> Tuple[bool, str]:
    """
    Set up a test directory with build configuration.
    
    Args:
        final_rs: Path to final.rs file
        example_dir: Path to original example directory
        example_name: Name of the example
        test_dir: Temporary test directory
    
    Returns: (success: bool, main_rs_path: str)
    """
    try:
        # Create Cargo.toml for binary compilation
        cargo_toml_dst = os.path.join(test_dir, "Cargo.toml")
        
        with open(cargo_toml_dst, "w") as f:
            f.write('''[package]
name = "output_test"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "output_test"
path = "src/main.rs"

[dependencies]
libc = "0.2"
lazy_static = "1.4"
once_cell = "1.18"
''')
        
        # Setup src directory
        src_dir = os.path.join(test_dir, "src")
        os.makedirs(src_dir, exist_ok=True)
        
        # Copy final.rs as main.rs
        main_rs_dst = os.path.join(src_dir, "main.rs")
        with open(final_rs, 'r') as f:
            final_content = f.read()
        
        with open(main_rs_dst, 'w') as f:
            f.write(final_content)
        
        return True, main_rs_dst
    
    except Exception as e:
        return False, str(e)


def run_output_test(test_dir: str, expected_output: str, timeout: int = TEST_TIMEOUT) -> OutputTestResult:
    """
    Run output verification test on prepared directory.
    
    1. Compile the program with cargo build
    2. Run the binary and capture output
    3. Compare with expected output
    
    Returns: OutputTestResult with test outcome
    """
    try:
        cargo_toml = os.path.join(test_dir, "Cargo.toml")
        target_dir = os.path.join(test_dir, "target")
        binary_path = os.path.join(target_dir, "debug", "output_test")
        
        # Clean previous build
        if os.path.exists(target_dir):
            shutil.rmtree(target_dir, ignore_errors=True)
        
        # Step 1: Build the program
        start = time.time()
        
        build_result = subprocess.run(
            ["cargo", "build", "--manifest-path", cargo_toml],
            capture_output=True,
            text=True,
            timeout=timeout // 2,  # Half timeout for build
            cwd=test_dir
        )
        
        if build_result.returncode != 0:
            return OutputTestResult(
                example_name="",
                success=False,
                message="❌ Build failed",
                details=build_result.stderr[:500],
                time_sec=0.0
            )
        
        # Step 2: Run the binary and capture output
        run_result = subprocess.run(
            [binary_path],
            capture_output=True,
            text=True,
            timeout=timeout // 2,  # Half timeout for execution
            cwd=test_dir
        )
        
        # Step 3: Compare output
        actual_output = run_result.stdout.strip()
        expected_trimmed = expected_output.strip()
        
        elapsed = time.time() - start
        
        if actual_output == expected_trimmed:
            return OutputTestResult(
                example_name="",
                success=True,
                message="✅ Output verification passed",
                time_sec=elapsed
            )
        else:
            # Output mismatch
            error_detail = f"Expected:\n{expected_trimmed}\n\nActual:\n{actual_output}"
            return OutputTestResult(
                example_name="",
                success=False,
                message="❌ Output mismatch",
                details=error_detail[:500],
                time_sec=elapsed
            )
    
    except subprocess.TimeoutExpired:
        return OutputTestResult(
            example_name="",
            success=False,
            message=f"⏱️  Timeout (>{timeout}s)",
            details="Test execution exceeded timeout",
            time_sec=timeout
        )
    except Exception as e:
        return OutputTestResult(
            example_name="",
            success=False,
            message=f"❌ {str(e)[:50]}",
            details=str(e),
            time_sec=0.0
        )


def test_one_example(
    example_name: str,
    final_rs: str,
    example_root_dir: str
) -> OutputTestResult:
    """
    Test one example for output verification.
    
    Sets up temp directory, compiles code, runs binary, and compares output.
    """
    test_dir = tempfile.mkdtemp(prefix=f"output_test_{example_name}_")
    
    try:
        # Find original example dir
        example_dir = find_example_dir(example_name)
        if not example_dir:
            return OutputTestResult(
                example_name=example_name,
                success=False,
                message="❌ Example dir not found"
            )
        
        # Read expected output from output.txt
        expected_output = ""
        output_txt_path = os.path.join(example_dir, "output.txt")
        if os.path.exists(output_txt_path):
            try:
                with open(output_txt_path, 'r') as f:
                    expected_output = f.read()
            except Exception as e:
                return OutputTestResult(
                    example_name=example_name,
                    success=False,
                    message=f"❌ Failed to read output.txt: {str(e)[:40]}"
                )
        else:
            return OutputTestResult(
                example_name=example_name,
                success=False,
                message="❌ output.txt not found"
            )
        
        # Setup test directory
        success, details = setup_output_test_dir(final_rs, example_dir, example_name, test_dir)
        if not success:
            return OutputTestResult(
                example_name=example_name,
                success=False,
                message=f"❌ Setup failed: {details[:50]}"
            )
        
        # Run test (compile and execute)
        result = run_output_test(test_dir, expected_output)
        result.example_name = example_name
        return result
    
    finally:
        shutil.rmtree(test_dir, ignore_errors=True)


def evaluate_output_directory(output_dir: str) -> Dict[str, OutputTestResult]:
    """
    Evaluate all examples in an output directory for output correctness.
    
    Returns: dict mapping example_name -> OutputTestResult
    """
    examples_dir = os.path.join(output_dir, "examples")
    if not os.path.exists(examples_dir):
        print(f"❌ Examples directory not found: {examples_dir}")
        return {}
    
    # Find all final.rs files
    final_files = glob.glob(os.path.join(examples_dir, "*/final.rs"))
    # Filter out negative examples (those with ____ in the name)
    final_files = [f for f in final_files if "____" not in os.path.basename(os.path.dirname(f))]
    total = len(final_files)
    
    print(f"🔍 Found {total} examples to test for output correctness")
    print()
    
    results = {}
    
    for i, final_rs in enumerate(sorted(final_files), 1):
        example_dir = os.path.dirname(final_rs)
        example_name = os.path.basename(example_dir)
        
        print(f"[{i:2d}/{total}] 🧪 Testing {example_name}...", end=" ", flush=True)
        
        result = test_one_example(example_name, final_rs, examples_dir)
        results[example_name] = result
        
        # Print status
        status = "✅" if result.success else "❌"
        print(f"{status} ({result.time_sec:.1f}s)")
        
        if result.message and result.message != "✅ Output verification passed":
            print(f"       {result.message}")
    
    return results


def generate_markdown_report(results: Dict[str, OutputTestResult], output_path: str = None) -> str:
    """
    Generate a readable markdown table report from output verification results.
    
    Returns: markdown string (and optionally writes to file)
    """
    timestamp = datetime.now().isoformat()
    
    lines = []
    w = lines.append
    
    w("# Output Verification Report")
    w("")
    w(f"**Generated:** {timestamp}")
    w(f"**Total Examples:** {len(results)}")
    w(f"**Passed:** {sum(1 for r in results.values() if r.success)}")
    w(f"**Failed:** {sum(1 for r in results.values() if not r.success)}")
    w("")
    
    # Summary statistics
    passed_count = sum(1 for r in results.values() if r.success)
    total_time = sum(r.time_sec for r in results.values())
    
    w("## Summary")
    w("")
    w(f"- **Pass Rate:** {passed_count}/{len(results)} ({100*passed_count/len(results):.1f}%)")
    w(f"- **Total Time:** {total_time:.1f}s")
    if len(results) > 0:
        w(f"- **Average Time:** {total_time/len(results):.1f}s per example")
    w("")
    
    # Detailed results table
    w("## Detailed Results")
    w("")
    w("| Example | Status | Message | Time (s) |")
    w("|---------|--------|---------|----------|")
    
    for example_name in sorted(results.keys()):
        result = results[example_name]
        status = "✅ PASS" if result.success else "❌ FAIL"
        message = result.message.replace("|", "\\|")[:60]
        time_str = f"{result.time_sec:.1f}"
        
        w(f"| `{example_name}` | {status} | {message} | {time_str} |")
    
    w("")
    
    # Failures section (if any)
    failures = {k: v for k, v in results.items() if not v.success}
    if failures:
        w("## Failed Examples")
        w("")
        
        for example_name in sorted(failures.keys()):
            result = failures[example_name]
            w(f"### {example_name}")
            w("")
            w("```")
            w(f"Message: {result.message}")
            if result.details:
                w(f"\nDetails:\n{result.details}")
            w("```")
            w("")
    
    # Passed section (if any)
    passed = {k: v for k, v in results.items() if v.success}
    if passed:
        w("## Passed Examples")
        w("")
        w("| Example | Time (s) |")
        w("|---------|----------|")
        
        for example_name in sorted(passed.keys()):
            result = passed[example_name]
            w(f"| `{example_name}` | {result.time_sec:.1f} |")
        
        w("")
    
    markdown = "\n".join(lines)
    
    # Write to file if requested
    if output_path:
        os.makedirs(os.path.dirname(output_path) or ".", exist_ok=True)
        with open(output_path, "w") as f:
            f.write(markdown)
        print(f"✅ Report written to: {output_path}")
    
    return markdown


def main():
    parser = argparse.ArgumentParser(
        description="Run output verification tests on LLM-generated Rust code"
    )
    parser.add_argument(
        "--output-dir",
        help="Path to output directory (with examples/*/final.rs)",
        default=None
    )
    parser.add_argument(
        "--from-last",
        action="store_true",
        help="Use the last refactor output directory (from .last_refactor_output)"
    )
    parser.add_argument(
        "--report-output",
        help="Path to save markdown report",
        default=None
    )
    parser.add_argument(
        "--json-output",
        help="Path to save JSON results",
        default=None
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=TEST_TIMEOUT,
        help=f"Timeout per test in seconds (default: {TEST_TIMEOUT})"
    )
    
    args = parser.parse_args()
    
    # Determine output directory
    output_dir = args.output_dir
    LLM_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    
    if args.from_last:
        last_output_file = os.path.join(LLM_DIR, ".last_refactor_output")
        if os.path.exists(last_output_file):
            with open(last_output_file) as f:
                output_dir = f.read().strip()
        else:
            print(f"❌ .last_refactor_output not found at {last_output_file}")
            sys.exit(1)
    
    if not output_dir:
        print("❌ Must specify --output-dir or use --from-last")
        parser.print_help()
        sys.exit(1)
    
    if not os.path.exists(output_dir):
        print(f"❌ Output directory not found: {output_dir}")
        sys.exit(1)
    
    print(f"📁 Testing directory: {output_dir}")
    print()
    
    # Run evaluation
    results = evaluate_output_directory(output_dir)
    
    print()
    print("=" * 80)
    print()
    
    # Generate report
    markdown = generate_markdown_report(results)
    print(markdown)
    
    # Save reports if requested
    if args.report_output:
        generate_markdown_report(results, args.report_output)
    
    if args.json_output:
        os.makedirs(os.path.dirname(args.json_output) or ".", exist_ok=True)
        json_data = {
            "timestamp": datetime.now().isoformat(),
            "total": len(results),
            "passed": sum(1 for r in results.values() if r.success),
            "failed": sum(1 for r in results.values() if not r.success),
            "results": {k: v.to_dict() for k, v in results.items()}
        }
        with open(args.json_output, "w") as f:
            json.dump(json_data, f, indent=2)
        print(f"✅ JSON results written to: {args.json_output}")
    
    # Exit with appropriate code
    if all(r.success for r in results.values()):
        print("✅ All examples passed output verification!")
        sys.exit(0)
    else:
        failed_count = sum(1 for r in results.values() if not r.success)
        print(f"❌ {failed_count} examples failed output verification")
        sys.exit(1)


if __name__ == "__main__":
    main()
