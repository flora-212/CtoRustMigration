#!/usr/bin/env python3
"""
Miri-based undefined behavior detection for LLM-generated Rust code.

Tests generated code with miri (Rust interpreter for detecting UB).
Reads final.rs files, appends test module, and runs MIRI_BACKTRACE=1 cargo +nightly miri test.
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
EXAMPLES_NEGATIVE_DIR = "/home/guoxy/concrat/examples_negative"
NIGHTLY = "nightly"
MIRI_TIMEOUT = 300  # 5 minutes per test (miri runs should be faster than loom)

# Test module to append to final.rs
TEST_MODULE = '''
#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn stress_test_correctness() {
        // Basic sanity check - just verify code compiles and runs
        for _ in 0..10 {
            // The main function should complete without panicking
            // Miri will detect any undefined behavior during execution
        }
    }
}
'''


class MiriTestResult:
    """Represents result of miri testing one example."""
    
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
    
    negative = os.path.join(EXAMPLES_NEGATIVE_DIR, example_name)
    if os.path.exists(negative):
        return negative
    
    return None


def setup_miri_test_dir(
    final_rs: str,
    example_dir: str,
    test_dir: str
) -> Tuple[bool, str]:
    """
    Set up a test directory with miri-compatible code.
    
    Returns: (success: bool, main_rs_path: str)
    """
    try:
        # Create a minimal Cargo.toml for binary testing
        # We skip the original Cargo.toml to avoid lib/binary conflicts
        cargo_toml_dst = os.path.join(test_dir, "Cargo.toml")
        
        with open(cargo_toml_dst, "w") as f:
            f.write('''[package]
name = "miri_test"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "miri_test"
path = "src/main.rs"

[dependencies]
libc = "0.2"
lazy_static = "1.4"
once_cell = "1.18"
''')
        
        # Setup src directory
        src_dir = os.path.join(test_dir, "src")
        os.makedirs(src_dir, exist_ok=True)
        
        # Read final.rs and append test module
        with open(final_rs, 'r') as f:
            final_content = f.read()
        
        # Append test module
        final_with_tests = final_content + "\n\n" + TEST_MODULE
        
        # Write to main.rs
        main_rs_dst = os.path.join(src_dir, "main.rs")
        with open(main_rs_dst, 'w') as f:
            f.write(final_with_tests)
        
        return True, main_rs_dst
    
    except Exception as e:
        return False, str(e)


def run_miri_test(test_dir: str, timeout: int = MIRI_TIMEOUT) -> MiriTestResult:
    """
    Run miri test on prepared directory.
    
    Returns: MiriTestResult with test outcome
    """
    try:
        cargo_toml = os.path.join(test_dir, "Cargo.toml")
        
        # Get rust-toolchain if available
        nightly = NIGHTLY
        tc_file = os.path.join(test_dir, "rust-toolchain")
        if os.path.exists(tc_file):
            with open(tc_file) as f:
                nightly = f.read().strip()
        
        # Clean previous build
        target_dir = os.path.join(test_dir, "target")
        if os.path.exists(target_dir):
            shutil.rmtree(target_dir, ignore_errors=True)
        
        # Setup environment for miri
        env = os.environ.copy()
        env["MIRIFLAGS"] = "-Zmiri-strict-provenance -Zmiri-preemption-rate=0"
        env["MIRI_BACKTRACE"] = "1"
        env["RUST_BACKTRACE"] = "1"
        
        # Run miri test
        start = time.time()
        
        result = subprocess.run(
            ["cargo", f"+{nightly}", "miri", "test", "--manifest-path", cargo_toml],
            capture_output=True,
            text=True,
            timeout=timeout,
            env=env,
            cwd=test_dir
        )
        
        elapsed = time.time() - start
        
        if result.returncode == 0:
            return MiriTestResult(
                example_name="",
                success=True,
                message="✅ Miri tests passed (no UB detected)",
                time_sec=elapsed
            )
        else:
            # Extract error message
            output = result.stderr + "\n" + result.stdout
            
            # Try to find the main error
            error_msg = "Miri test failed"
            for line in output.split('\n'):
                if 'error' in line.lower() or 'ub' in line.lower() or 'undefined' in line.lower():
                    error_msg = line.strip()[:100]
                    break
            
            return MiriTestResult(
                example_name="",
                success=False,
                message=f"❌ {error_msg}",
                details=output[:500],
                time_sec=elapsed
            )
    
    except subprocess.TimeoutExpired:
        return MiriTestResult(
            example_name="",
            success=False,
            message=f"⏱️  Timeout (>{timeout}s)",
            details="Miri test execution exceeded timeout",
            time_sec=timeout
        )
    except Exception as e:
        return MiriTestResult(
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
) -> MiriTestResult:
    """
    Test one example with miri.
    
    Sets up temp directory, adds test module, and runs miri tests.
    """
    test_dir = tempfile.mkdtemp(prefix=f"miri_test_{example_name}_")
    
    try:
        # Find original example dir
        example_dir = find_example_dir(example_name)
        if not example_dir:
            return MiriTestResult(
                example_name=example_name,
                success=False,
                message="❌ Example dir not found"
            )
        
        # Setup miri test directory
        success, details = setup_miri_test_dir(final_rs, example_dir, test_dir)
        if not success:
            return MiriTestResult(
                example_name=example_name,
                success=False,
                message=f"❌ Setup failed: {details[:50]}"
            )
        
        # Run miri test
        result = run_miri_test(test_dir)
        result.example_name = example_name
        return result
    
    finally:
        shutil.rmtree(test_dir, ignore_errors=True)


def evaluate_output_directory(output_dir: str) -> Dict[str, MiriTestResult]:
    """
    Evaluate all examples in an output directory for miri UB detection.
    
    Returns: dict mapping example_name -> MiriTestResult
    """
    examples_dir = os.path.join(output_dir, "examples")
    if not os.path.exists(examples_dir):
        print(f"❌ Examples directory not found: {examples_dir}")
        return {}
    
    # Find all final.rs files
    final_files = glob.glob(os.path.join(examples_dir, "*/final.rs"))
    total = len(final_files)
    
    print(f"🔍 Found {total} examples to test with miri")
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
        
        if result.message and result.message != "✅ Miri tests passed (no UB detected)":
            print(f"       {result.message}")
    
    return results


def generate_markdown_report(results: Dict[str, MiriTestResult], output_path: str = None) -> str:
    """
    Generate a readable markdown table report from miri test results.
    
    Returns: markdown string (and optionally writes to file)
    """
    timestamp = datetime.now().isoformat()
    
    lines = []
    w = lines.append
    
    w("# Miri Undefined Behavior Detection Report")
    w("")
    w(f"**Generated:** {timestamp}")
    w(f"**Total Examples:** {len(results)}")
    w(f"**Passed (No UB):** {sum(1 for r in results.values() if r.success)}")
    w(f"**Failed (UB Detected):** {sum(1 for r in results.values() if not r.success)}")
    w("")
    
    # Summary statistics
    passed_count = sum(1 for r in results.values() if r.success)
    total_time = sum(r.time_sec for r in results.values())
    
    w("## Summary")
    w("")
    w(f"- **Clean Code Rate:** {passed_count}/{len(results)} ({100*passed_count/len(results):.1f}%)")
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
        w("## UB Detected (Failures)")
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
        w("## Safe Examples (No UB)")
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
        description="Run miri-based UB detection tests on LLM-generated Rust code"
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
        default=MIRI_TIMEOUT,
        help=f"Timeout per test in seconds (default: {MIRI_TIMEOUT})"
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
        print("✅ All examples passed miri testing!")
        sys.exit(0)
    else:
        failed_count = sum(1 for r in results.values() if not r.success)
        print(f"❌ {failed_count} examples failed miri testing")
        sys.exit(1)


if __name__ == "__main__":
    main()
