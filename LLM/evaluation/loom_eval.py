#!/usr/bin/env python3
"""
Loom-based concurrency testing for LLM-generated Rust code.

Tests generated code with loom (state space exploration for concurrency).
Converts final.rs files and runs RUSTFLAGS="--cfg loom" cargo test.
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

# Add LLM to path to find loom_converter
LLM_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if LLM_DIR not in sys.path:
    sys.path.insert(0, LLM_DIR)

try:
    from loom_converter import convert_file as loom_convert_file
except ImportError as e:
    print(f"⚠️  Warning: Could not import loom_converter: {e}")
    loom_convert_file = None

# Paths
EXAMPLES_DIR = "/home/guoxy/concrat/examples"
EXAMPLES_NEGATIVE_DIR = "/home/guoxy/concrat/examples_negative"
NIGHTLY = "nightly"
LOOM_TIMEOUT = 600  # 10 minutes per test (loom can be very slow)


class LoomTestResult:
    """Represents result of loom testing one example."""
    
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


def setup_loom_test_dir(
    final_rs: str,
    example_dir: str,
    test_dir: str
) -> Tuple[bool, str]:
    """
    Set up a test directory with loom-compatible code.
    
    Returns: (success: bool, main_rs_path: str)
    """
    try:
        # Copy Cargo.toml
        cargo_toml_src = os.path.join(example_dir, "Cargo.toml")
        cargo_toml_dst = os.path.join(test_dir, "Cargo.toml")
        if os.path.exists(cargo_toml_src):
            with open(cargo_toml_src) as f:
                content = f.read()
            
            # Fix lib path from "c2rust-lib.rs" to "src/lib.rs"
            content = content.replace('path = "c2rust-lib.rs"', 'path = "src/lib.rs"')
            
            # Ensure loom is in dev-dependencies
            if "[dev-dependencies]" not in content:
                content += '\n[dev-dependencies]\nloom = "0.7"\n'
            elif "loom" not in content:
                lines = content.split('\n')
                dev_idx = next(i for i, line in enumerate(lines) if "[dev-dependencies]" in line)
                lines.insert(dev_idx + 1, 'loom = "0.7"')
                content = '\n'.join(lines)
            
            with open(cargo_toml_dst, "w") as f:
                f.write(content)
        else:
            # Create minimal Cargo.toml with lib definition for c2rust-lib.rs
            lib_config = ""
            if os.path.exists(os.path.join(example_dir, "c2rust-lib.rs")):
                lib_config = '''
[lib]
name = "c2rust_out"
path = "src/lib.rs"
crate-type = ["staticlib", "rlib"]
'''
            
            with open(cargo_toml_dst, "w") as f:
                f.write(f'''[package]
name = "loom_test"
version = "0.1.0"
edition = "2021"
{lib_config}
[dependencies]
libc = "0.2"

[dev-dependencies]
loom = "0.7"
''')
        
        # Copy rust-toolchain if available
        tc_src = os.path.join(example_dir, "rust-toolchain")
        if os.path.exists(tc_src):
            shutil.copy(tc_src, os.path.join(test_dir, "rust-toolchain"))
        
        # Copy c2rust-lib.rs or create stub
        lib_rs_src = os.path.join(example_dir, "c2rust-lib.rs")
        lib_rs_dst = os.path.join(test_dir, "src", "lib.rs")
        os.makedirs(os.path.dirname(lib_rs_dst), exist_ok=True)
        
        if os.path.exists(lib_rs_src):
            shutil.copy(lib_rs_src, lib_rs_dst)
        elif os.path.exists(cargo_toml_src):
            # Original Cargo.toml existed, may reference lib.rs
            # Create a minimal stub to avoid missing module errors
            with open(lib_rs_dst, "w") as f:
                f.write("""#![allow(dead_code)]
#![allow(unused_imports)]
extern crate libc;

// Re-export all items from main module for testing
pub use main::*;
pub mod main;
""")


        
        # Convert final.rs with loom_converter
        main_rs_dst = os.path.join(test_dir, "src", "main.rs")
        
        if loom_convert_file:
            try:
                # Use loom_converter to convert the file
                loom_convert_file(
                    final_rs,
                    main_rs_dst,
                    example_dir=example_dir,
                    standalone=True
                )
            except Exception as e:
                # Fallback: use original file
                print(f"     ⚠️  loom_converter failed: {e}, using original file")
                shutil.copy(final_rs, main_rs_dst)
        else:
            # No converter available - just copy original
            shutil.copy(final_rs, main_rs_dst)
        
        return True, main_rs_dst
    
    except Exception as e:
        return False, str(e)


def run_loom_test(test_dir: str, timeout: int = LOOM_TIMEOUT) -> LoomTestResult:
    """
    Run loom test on prepared directory.
    
    Returns: LoomTestResult with test outcome
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
        
        # Setup environment
        env = os.environ.copy()
        env["LOOM_CHECKPOINT_INTERVAL"] = "100"
        env["CARGO_INCREMENTAL"] = "1"
        env["RAYON_NUM_THREADS"] = "4"
        env["RUSTFLAGS"] = "--cfg loom"
        
        # Run loom test
        import time
        start = time.time()
        
        result = subprocess.run(
            ["cargo", f"+{nightly}", "test", "--manifest-path", cargo_toml],
            capture_output=True,
            text=True,
            timeout=timeout,
            env=env,
            cwd=test_dir
        )
        
        elapsed = time.time() - start
        
        if result.returncode == 0:
            return LoomTestResult(
                example_name="",
                success=True,
                message="✅ Loom tests passed",
                time_sec=elapsed
            )
        else:
            # Extract error message
            output = result.stderr + "\n" + result.stdout
            
            # Try to find the main error
            error_msg = "Loom test failed"
            for line in output.split('\n'):
                if 'error' in line.lower() or 'panic' in line.lower():
                    error_msg = line.strip()[:100]
                    break
            
            return LoomTestResult(
                example_name="",
                success=False,
                message=f"❌ {error_msg}",
                details=output[:500],
                time_sec=elapsed
            )
    
    except subprocess.TimeoutExpired:
        return LoomTestResult(
            example_name="",
            success=False,
            message=f"⏱️  Timeout (>{timeout}s)",
            details="Loom state space exploration exceeded timeout",
            time_sec=timeout
        )
    except Exception as e:
        return LoomTestResult(
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
) -> LoomTestResult:
    """
    Test one example with loom.
    
    Sets up temp directory, converts code, and runs tests.
    """
    test_dir = tempfile.mkdtemp(prefix=f"loom_test_{example_name}_")
    
    try:
        # Find original example dir
        example_dir = find_example_dir(example_name)
        if not example_dir:
            return LoomTestResult(
                example_name=example_name,
                success=False,
                message="❌ Example dir not found"
            )
        
        # Setup loom test directory
        success, details = setup_loom_test_dir(final_rs, example_dir, test_dir)
        if not success:
            return LoomTestResult(
                example_name=example_name,
                success=False,
                message=f"❌ Setup failed: {details[:50]}"
            )
        
        # Run loom test
        result = run_loom_test(test_dir)
        result.example_name = example_name
        return result
    
    finally:
        shutil.rmtree(test_dir, ignore_errors=True)


def evaluate_output_directory(output_dir: str) -> Dict[str, LoomTestResult]:
    """
    Evaluate all examples in an output directory for loom compatibility.
    
    Returns: dict mapping example_name -> LoomTestResult
    """
    examples_dir = os.path.join(output_dir, "examples")
    if not os.path.exists(examples_dir):
        print(f"❌ Examples directory not found: {examples_dir}")
        return {}
    
    # Find all final.rs files
    final_files = glob.glob(os.path.join(examples_dir, "*/final.rs"))
    total = len(final_files)
    
    print(f"🔍 Found {total} examples to test")
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
        
        if result.message and result.message != ("✅ Loom tests passed"):
            print(f"       {result.message}")
    
    return results


def generate_markdown_report(results: Dict[str, LoomTestResult], output_path: str = None) -> str:
    """
    Generate a readable markdown table report from loom test results.
    
    Returns: markdown string (and optionally writes to file)
    """
    timestamp = datetime.now().isoformat()
    
    lines = []
    w = lines.append
    
    w("# Loom Concurrency Test Report")
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
        w("## Failures")
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
        description="Run loom-based concurrency tests on LLM-generated Rust code"
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
        default=LOOM_TIMEOUT,
        help=f"Timeout per test in seconds (default: {LOOM_TIMEOUT})"
    )
    
    args = parser.parse_args()
    
    # Determine output directory
    output_dir = args.output_dir
    
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
    
    # Generate markdown report
    markdown = generate_markdown_report(results)
    print(markdown)
    
    # Save markdown report
    if args.report_output:
        generate_markdown_report(results, args.report_output)
    else:
        # Default: save to evaluation directory
        eval_dir = os.path.join(output_dir, "evaluation")
        os.makedirs(eval_dir, exist_ok=True)
        report_path = os.path.join(eval_dir, "loom_test_report.md")
        with open(report_path, "w") as f:
            f.write(markdown)
        print(f"\n✅ Report saved to: {report_path}")
    
    # Save JSON results
    if args.json_output:
        json_data = {
            "timestamp": datetime.now().isoformat(),
            "total": len(results),
            "passed": sum(1 for r in results.values() if r.success),
            "failed": sum(1 for r in results.values() if not r.success),
            "results": {k: v.to_dict() for k, v in results.items()}
        }
        os.makedirs(os.path.dirname(args.json_output) or ".", exist_ok=True)
        with open(args.json_output, "w") as f:
            json.dump(json_data, f, indent=2)
        print(f"✅ JSON results saved to: {args.json_output}")
    else:
        # Default JSON too
        eval_dir = os.path.join(output_dir, "evaluation")
        os.makedirs(eval_dir, exist_ok=True)
        json_path = os.path.join(eval_dir, "loom_test_results.json")
        json_data = {
            "timestamp": datetime.now().isoformat(),
            "total": len(results),
            "passed": sum(1 for r in results.values() if r.success),
            "failed": sum(1 for r in results.values() if not r.success),
            "results": {k: v.to_dict() for k, v in results.items()}
        }
        with open(json_path, "w") as f:
            json.dump(json_data, f, indent=2)
        print(f"✅ JSON results saved to: {json_path}")


if __name__ == "__main__":
    main()
