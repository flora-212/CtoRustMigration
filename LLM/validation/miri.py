#!/usr/bin/env python3
"""
Miri validation tool.

Provides undefined behavior detection via cargo miri.
Supports cargo-nextest for parallel execution.
"""

import os
import subprocess
import tempfile
import shutil
from typing import Optional
from .errors import ErrorInfo, ValidationResult
from .utils import parse_compile_errors


class MiriValidator:
    """Validator for undefined behavior detection with miri."""
    
    NIGHTLY = "nightly"
    MIRI_TIMEOUT = 1800  # 30 minutes
    
    @staticmethod
    def validate_miri(rs_file: str, example_dir: Optional[str] = None, nightly: str = NIGHTLY) -> ValidationResult:
        """
        Validate code with miri (detects UB at runtime).
        
        Uses cargo-nextest for parallel test execution (faster) if available,
        falls back to cargo miri test (slower) otherwise.
        
        Performance: nextest can be 3-5x faster due to parallelism
        Reference: https://github.com/rust-lang/miri/blob/master/README.md
        """
        td = tempfile.mkdtemp()
        try:
            # Copy Cargo.toml and support files
            for f in ["Cargo.toml", "rust-toolchain"]:
                src = os.path.join(example_dir, f) if example_dir else None
                if src and os.path.exists(src):
                    shutil.copy(src, td)
            
            # Copy c2rust-lib.rs if available
            lib_rs = os.path.join(example_dir, "c2rust-lib.rs") if example_dir else None
            if lib_rs and os.path.exists(lib_rs):
                shutil.copy(lib_rs, td)
            else:
                with open(os.path.join(td, "c2rust-lib.rs"), "w") as f:
                    f.write("#![allow(dead_code)]\n")
                    f.write("extern crate libc;\n")
                    f.write("pub mod main;\n")
            
            # Copy the source file as main.rs
            src_main_rs = os.path.join(td, "main.rs")
            shutil.copy(rs_file, src_main_rs)
            
            # Read rust-toolchain if exists
            tc_file = os.path.join(td, "rust-toolchain")
            if os.path.exists(tc_file):
                with open(tc_file) as f:
                    nightly = f.read().strip()
            
            # Clean build artifacts
            cargo_lock = os.path.join(td, "Cargo.lock")
            if os.path.exists(cargo_lock):
                os.remove(cargo_lock)
            
            target_dir = os.path.join(td, "target")
            if os.path.exists(target_dir):
                shutil.rmtree(target_dir, ignore_errors=True)
            
            compile_env = {**os.environ, "MIRIFLAGS": "-Zmiri-strict-provenance"}
            compile_env["CARGO_INCREMENTAL"] = "1"
            compile_env["RAYON_NUM_THREADS"] = "4"
            
            # Try nextest first (faster - parallelizes test runs)
            has_nextest = False
            try:
                subprocess.run(
                    ["cargo", f"+{nightly}", "nextest", "--version"],
                    capture_output=True, timeout=5
                )
                has_nextest = True
            except (subprocess.TimeoutExpired, FileNotFoundError):
                has_nextest = False
            
            if has_nextest:
                # Use nextest for parallel execution (3-5x faster)
                result = subprocess.run(
                    ["cargo", f"+{nightly}", "miri", "nextest", "run", "-j4",
                     "--manifest-path", os.path.join(td, "Cargo.toml")],
                    capture_output=True, text=True, timeout=MiriValidator.MIRI_TIMEOUT,
                    env=compile_env
                )
            else:
                # Fallback to cargo miri test (slower - single threaded)
                result = subprocess.run(
                    ["cargo", f"+{nightly}", "miri", "test", "--manifest-path",
                     os.path.join(td, "Cargo.toml")],
                    capture_output=True, text=True, timeout=MiriValidator.MIRI_TIMEOUT,
                    env=compile_env
                )
            
            if result.returncode == 0:
                return ValidationResult(True, "miri", "Miri analysis passed (no UB detected)", errors=[])
            else:
                errors = parse_compile_errors(result.stderr)
                if not errors:
                    errors = [ErrorInfo(
                        error_type="miri_error",
                        message="Miri detected potential undefined behavior",
                        details=result.stderr[:1000]
                    )]
                
                error_summary = "\n".join([str(e) for e in errors[:3]])
                error_dicts = [e.to_dict() for e in errors]
                return ValidationResult(
                    False, "miri", f"Miri found {len(errors)} issue(s)",
                    details={"errors": error_dicts, "summary": error_summary},
                    errors=errors
                )
        
        except subprocess.TimeoutExpired:
            return ValidationResult(
                False, "miri",
                f"Miri timeout (>{MiriValidator.MIRI_TIMEOUT}s)",
                errors=[ErrorInfo(
                    error_type="miri_error",
                    message=f"Miri timeout (>{MiriValidator.MIRI_TIMEOUT}s)",
                    details="Miri analysis took too long (>30 minutes). This suggests infinite loops or very complex code patterns."
                )]
            )
        except Exception as e:
            return ValidationResult(
                False, "miri", str(e),
                errors=[ErrorInfo(
                    error_type="miri_error",
                    message=str(e),
                    details=str(e)[:300]
                )]
            )
        finally:
            shutil.rmtree(td, ignore_errors=True)
