#!/usr/bin/env python3
"""
Debugging script for validating validators.

Tests all validators in the validation module against target files.
Usage: python3 debug_validators.py <rs_file> [--example-dir <dir>]
"""

import sys
import os
import argparse
import subprocess
from pathlib import Path
from typing import Optional

# Add parent directory to path for imports
validation_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(validation_dir))

# Import the validation module properly
from validation.core import CodeValidator
from validation.errors import ValidationResult


class ValidatorDebugger:
    """Debug and test all validators."""
    
    def __init__(self, rs_file: str, example_dir: Optional[str] = None):
        """
        Initialize debugger.
        
        Args:
            rs_file: Path to Rust file to validate
            example_dir: Path to example directory (optional)
        """
        self.rs_file = rs_file
        self.example_dir = example_dir or os.path.dirname(rs_file)
        self.validator = CodeValidator()
        self.main_rs_path = os.path.join(self.example_dir, "main.rs")
        self.tests_rs_path = os.path.join(self.example_dir, "tests.rs")
        
        # Validate that the file exists
        if not os.path.exists(self.rs_file):
            raise FileNotFoundError(f"File not found: {self.rs_file}")
        
        if not os.path.exists(self.example_dir):
            raise FileNotFoundError(f"Example directory not found: {self.example_dir}")
    
    def _prepare_main_rs(self, append_tests: bool = False) -> None:
        """
        Prepare main.rs by copying final.rs and optionally appending tests.
        
        Args:
            append_tests: If True, append tests.rs content to main.rs
        """
        # Copy final.rs to main.rs
        with open(self.rs_file, 'r') as f:
            content = f.read()
        
        # If append_tests, add tests content after main code
        if append_tests and os.path.exists(self.tests_rs_path):
            with open(self.tests_rs_path, 'r') as f:
                tests_content = f.read()
            content += "\n\n" + tests_content
        
        with open(self.main_rs_path, 'w') as f:
            f.write(content)
    
    def _cleanup_main_rs(self) -> None:
        """Clean up temporary main.rs file."""
        if os.path.exists(self.main_rs_path):
            os.remove(self.main_rs_path)
    
    def _prepare_loom_test(self) -> Optional[str]:
        """
        Prepare loom test using loom_converter.
        
        Returns:
            Path to the loom test file if successful, None otherwise.
        """
        # Locate loom_converter.py
        llm_dir = os.path.dirname(validation_dir)  # Go up to LLM directory
        loom_converter_path = os.path.join(llm_dir, "validation/loom_converter.py")
        
        if not os.path.exists(loom_converter_path):
            print(f"⚠️  Warning: loom_converter.py not found at {loom_converter_path}")
            return None
        
        # Create tests directory if it doesn't exist
        tests_dir = os.path.join(self.example_dir, "tests")
        os.makedirs(tests_dir, exist_ok=True)
        
        # Prepare loom test file path
        loom_test_path = os.path.join(tests_dir, "loom.rs")
        
        print(f"Preparing loom test using converter...")
        print(f"  Input: {self.rs_file}")
        print(f"  Output: {loom_test_path}")
        print(f"  Converter: {loom_converter_path}\n")
        
        # Run loom converter with --standalone flag
        try:
            cmd = [
                sys.executable,
                loom_converter_path,
                self.rs_file,
                loom_test_path,
                "--example-dir", self.example_dir,
                "--standalone"
            ]
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=10
            )
            
            if result.returncode != 0:
                print(f"❌ Loom converter failed:")
                print(f"  stderr: {result.stderr}")
                return None
            
            # Print converter output
            if result.stdout:
                for line in result.stdout.split('\n'):
                    if line.strip():
                        print(f"  {line}")
            
            if not os.path.exists(loom_test_path):
                print(f"❌ Loom test file not created at {loom_test_path}")
                return None
            
            print(f"✅ Loom test prepared successfully at {loom_test_path}\n")
            return loom_test_path
        
        except subprocess.TimeoutExpired:
            print(f"❌ Loom converter timed out")
            return None
        except Exception as e:
            print(f"❌ Error running loom converter: {e}")
            return None
    
    def _cleanup_loom_test(self, loom_test_path: Optional[str]) -> None:
        """
        Clean up loom test file.
        
        Args:
            loom_test_path: Path to loom test file to clean up
        """
        if loom_test_path and os.path.exists(loom_test_path):
            try:
                os.remove(loom_test_path)
            except OSError:
                pass
    
    def _run_loom_test(self, loom_test_path: str) -> ValidationResult:
        """
        Run the converted loom test using cargo.
        
        Args:
            loom_test_path: Path to the loom test file
            
        Returns:
            ValidationResult with the loom test outcome
        """
        print(f"Running loom test: {loom_test_path}\n")
        
        # Ensure main.rs exists (required by c2rust-lib.rs)
        # This can be a minimal stub since we're testing via tests/loom.rs
        if not os.path.exists(self.main_rs_path):
            print(f"Creating placeholder main.rs for cargo...\n")
            with open(self.main_rs_path, 'w') as f:
                f.write("// Placeholder for loom testing\nfn main() {}\n")
        
        try:
            # Prepare environment for loom testing
            env = os.environ.copy()
            env["RUSTFLAGS"] = "--cfg loom"
            env["LOOM_CHECKPOINT_INTERVAL"] = "100"
            env["CARGO_INCREMENTAL"] = "1"
            env["RAYON_NUM_THREADS"] = "4"
            
            # Get the test name from the file path
            test_name = os.path.splitext(os.path.basename(loom_test_path))[0]
            
            # Run: RUSTFLAGS="--cfg loom" cargo test --test loom --release
            cmd = [
                "cargo",
                "test",
                f"--test", test_name,
                "--release"
            ]
            
            print(f"  Command: {' '.join(cmd)}")
            print(f"  RUSTFLAGS: --cfg loom\n")
            
            result = subprocess.run(
                cmd,
                cwd=self.example_dir,
                capture_output=True,
                text=True,
                timeout=1800,  # 30 minutes for loom (it's slow!)
                env=env
            )
            
            if result.returncode == 0:
                return ValidationResult(
                    passed=True,
                    category="loom",
                    message="Loom analysis passed (no data races detected)",
                    errors=[]
                )
            else:
                # Parse error output
                error_msg = result.stderr + result.stdout
                
                # Check for common loom errors
                if "panicked" in error_msg:
                    return ValidationResult(
                        passed=False,
                        category="loom",
                        message="Loom test panicked (potential concurrency issue detected)",
                        errors=[],
                        details={"error": error_msg[:500]}
                    )
                else:
                    return ValidationResult(
                        passed=False,
                        category="loom",
                        message="Loom test failed",
                        errors=[],
                        details={"error": error_msg[:500]}
                    )
        
        except subprocess.TimeoutExpired:
            return ValidationResult(
                passed=False,
                category="loom",
                message="Loom test timed out (loom is very slow, try a simpler test)",
                errors=[]
            )
        except Exception as e:
            return ValidationResult(
                passed=False,
                category="loom",
                message=f"Error running loom test: {e}",
                errors=[]
            )
    
    def _print_section(self, title: str) -> None:
        """Print a formatted section header."""
        print(f"\n{'=' * 70}")
        print(f"  {title}")
        print(f"{'=' * 70}\n")
    
    def _print_result(self, result: ValidationResult, indent: int = 0) -> None:
        """Print a validation result nicely."""
        prefix = "  " * indent
        status = "✅ PASS" if result.passed else "❌ FAIL"
        print(f"{prefix}{status} | {result.category}")
        if result.message:
            print(f"{prefix}  Message: {result.message}")
        
        # Print error details if any
        if result.errors:
            print(f"{prefix}  Errors found: {len(result.errors)}")
            for i, error in enumerate(result.errors[:5], 1):  # Show first 5 errors
                print(f"{prefix}    [{i}] {error}")
            if len(result.errors) > 5:
                print(f"{prefix}    ... and {len(result.errors) - 5} more errors")
        
        # Print additional details
        if result.details:
            for key, value in result.details.items():
                print(f"{prefix}  {key}: {value}")
    
    def run_all_validators(self) -> None:
        """Run all available validators."""
        self._print_section("Validation Target")
        print(f"File: {self.rs_file}")
        print(f"Directory: {self.example_dir}")
        print(f"File Size: {os.path.getsize(self.rs_file)} bytes")
        
        results = {}
        
        # 1. Compile Validation
        self._print_section("1. COMPILE VALIDATOR")
        print("Running: cargo build (compilation check)...\n")
        try:
            self._prepare_main_rs(append_tests=False)
            compile_result = self.validator.validate_compile(self.main_rs_path, self.example_dir)
            self._print_result(compile_result)
            results['compile'] = compile_result
        finally:
            self._cleanup_main_rs()
        
        # 2. Clippy Validation
        self._print_section("2. CLIPPY VALIDATOR")
        print("Running: cargo clippy (linting check)...\n")
        try:
            self._prepare_main_rs(append_tests=False)
            clippy_result = self.validator.validate_clippy(self.main_rs_path, self.example_dir)
            self._print_result(clippy_result)
            results['clippy'] = clippy_result
        finally:
            self._cleanup_main_rs()
        
        # 3. Miri Validation (only if compilation passed)
        self._print_section("3. MIRI VALIDATOR")
        if compile_result.passed:
            print("Compilation passed, running: cargo miri (undefined behavior detection)...\n")
            try:
                self._prepare_main_rs(append_tests=True)
                miri_result = self.validator.validate_miri(self.main_rs_path, self.example_dir)
                self._print_result(miri_result)
                results['miri'] = miri_result
            finally:
                self._cleanup_main_rs()
        else:
            print("Skipping miri (compilation failed)")
            results['miri'] = None
        
        # 4. Loom Validation (only if compilation passed)
        self._print_section("4. LOOM VALIDATOR")
        if compile_result.passed:
            print("Compilation passed, preparing loom concurrency test...\n")
            loom_test_path = self._prepare_loom_test()
            
            if loom_test_path:
                try:
                    print("Step 2: Running loom validation...\n")
                    loom_result = self._run_loom_test(loom_test_path)
                    self._print_result(loom_result)
                    results['loom'] = loom_result
                finally:
                    self._cleanup_loom_test(loom_test_path)
            else:
                print("⚠️  Skipping loom validation (failed to prepare loom test)")
                results['loom'] = None
        else:
            print("Skipping loom (compilation failed)")
            results['loom'] = None
        
        # 5. Safety Validation
        self._print_section("5. SAFETY VALIDATOR")
        print("Running: static analysis (lock safety analysis)...\n")
        with open(self.rs_file, 'r') as f:
            code = f.read()
        safety_result = self.validator.validate_safety(code)
        self._print_result(safety_result)
        results['safety'] = safety_result
        
        # Summary
        self._print_section("VALIDATION SUMMARY")
        passed_count = sum(1 for r in results.values() if r and r.passed)
        total_count = len([r for r in results.values() if r])
        
        print(f"Validators Run: {total_count}/{len(results)}")
        print(f"Passed: {passed_count}/{total_count}")
        print(f"\nDetailed Results:")
        
        for name, result in results.items():
            if result:
                status = "✅" if result.passed else "❌"
                print(f"  {status} {name.upper()}: {result.message or 'OK'}")
            else:
                print(f"  ⊘  {name.upper()}: Skipped")
        
        # Overall verdict
        print("\n" + "=" * 70)
        if passed_count == total_count:
            print("  ✅ ALL VALIDATIONS PASSED!")
        else:
            print(f"  ❌ VALIDATION FAILED ({total_count - passed_count} failures)")
        print("=" * 70 + "\n")
    
    def run_single_validator(self, validator_name: str) -> None:
        """Run a single validator by name."""
        validator_name = validator_name.lower()
        
        loom_test_path = None
        try:
            if validator_name == 'compile':
                self._print_section("COMPILE VALIDATOR")
                self._prepare_main_rs(append_tests=False)
                result = self.validator.validate_compile(self.main_rs_path, self.example_dir)
            elif validator_name == 'clippy':
                self._print_section("CLIPPY VALIDATOR")
                self._prepare_main_rs(append_tests=False)
                result = self.validator.validate_clippy(self.main_rs_path, self.example_dir)
            elif validator_name == 'miri':
                self._print_section("MIRI VALIDATOR")
                self._prepare_main_rs(append_tests=True)
                result = self.validator.validate_miri(self.main_rs_path, self.example_dir)
            elif validator_name == 'loom':
                self._print_section("LOOM VALIDATOR")
                print("Step 1: Preparing loom test using converter...\n")
                loom_test_path = self._prepare_loom_test()
                
                if loom_test_path:
                    print("Step 2: Running loom validation...\n")
                    result = self._run_loom_test(loom_test_path)
                else:
                    # Create a dummy failed result
                    result = ValidationResult(
                        passed=False,
                        category="loom",
                        message="Failed to prepare loom test using converter",
                        errors=[],
                        details={}
                    )
            elif validator_name == 'safety':
                self._print_section("SAFETY VALIDATOR")
                with open(self.rs_file, 'r') as f:
                    code = f.read()
                result = self.validator.validate_safety(code)
            else:
                print(f"Unknown validator: {validator_name}")
                print("Available validators: compile, clippy, miri, loom, safety")
                return
            
            self._print_result(result)
            
            # Print detailed error information
            if result.errors:
                print("\nDetailed Errors:")
                for i, error in enumerate(result.errors, 1):
                    print(f"\n  Error {i}:")
                    print(f"    Type: {error.error_type}")
                    print(f"    Message: {error.message}")
                    print(f"    Location: {error.location}")
                    print(f"    Code: {error.error_code}")
                    print(f"    Line: {error.line}, Column: {error.column}")
                    if error.details:
                        print(f"    Details: {error.details[:200]}...")
        finally:
            # self._cleanup_main_rs()
            self._cleanup_loom_test(loom_test_path)


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Debug and test validators for Rust code."
    )
    parser.add_argument(
        'rs_file',
        help='Path to Rust file to validate'
    )
    parser.add_argument(
        '--example-dir',
        help='Path to example directory (defaults to parent of rs_file)',
        default=None
    )
    parser.add_argument(
        '--validator',
        help='Run only a specific validator (compile, clippy, miri, loom, safety)',
        default=None
    )
    
    args = parser.parse_args()
    
    # Convert relative paths to absolute
    rs_file = os.path.abspath(args.rs_file)
    example_dir = os.path.abspath(args.example_dir) if args.example_dir else None
    
    try:
        debugger = ValidatorDebugger(rs_file, example_dir)
        
        if args.validator:
            debugger.run_single_validator(args.validator)
        else:
            debugger.run_all_validators()
    
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()
