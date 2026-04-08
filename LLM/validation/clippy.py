#!/usr/bin/env python3
"""
Clippy validation tool.

Provides linting and code quality checks via cargo clippy.
"""

import os
import subprocess
import tempfile
import shutil
from typing import Optional
from .errors import ErrorInfo, ValidationResult
from .utils import parse_compile_errors


class ClippyValidator:
    """Validator for code linting with clippy."""
    
    NIGHTLY = "nightly"
    COMPILE_TIMEOUT = 600
    
    @staticmethod
    def validate_clippy(rs_file: str, example_dir: Optional[str] = None, nightly: str = NIGHTLY) -> ValidationResult:
        """Validate code with clippy linter."""
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
            
            compile_env = {**os.environ, "RUSTFLAGS": "-Awarnings"}
            compile_env["CARGO_INCREMENTAL"] = "1"
            compile_env["RAYON_NUM_THREADS"] = "4"
            compile_env["CARGO_NET_RETRY"] = "3"
            
            result = subprocess.run(
                ["cargo", f"+{nightly}", "clippy", "--manifest-path",
                 os.path.join(td, "Cargo.toml"), "--", "-D", "warnings"],
                capture_output=True, text=True, timeout=ClippyValidator.COMPILE_TIMEOUT,
                env=compile_env
            )
            
            if result.returncode == 0:
                return ValidationResult(True, "clippy", "Clippy checks passed", errors=[])
            else:
                errors = parse_compile_errors(result.stderr)
                if not errors:
                    errors = [ErrorInfo(
                        error_type="clippy_error",
                        message="Clippy warnings found",
                        details=result.stderr[:1000]
                    )]
                return ValidationResult(False, "clippy", f"Clippy found {len(errors)} issue(s)", errors=errors)
        
        except subprocess.TimeoutExpired:
            return ValidationResult(
                False, "clippy",
                f"Clippy timeout (>{ClippyValidator.COMPILE_TIMEOUT}s)",
                errors=[ErrorInfo(
                    error_type="clippy_error",
                    message=f"Clippy timeout (>{ClippyValidator.COMPILE_TIMEOUT}s)",
                    details="Clippy analysis took too long"
                )]
            )
        except Exception as e:
            return ValidationResult(
                False, "clippy", str(e),
                errors=[ErrorInfo(
                    error_type="clippy_error",
                    message=str(e),
                    details=str(e)[:300]
                )]
            )
        finally:
            shutil.rmtree(td, ignore_errors=True)
