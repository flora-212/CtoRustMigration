#!/usr/bin/env python3
"""
Compilation validation tool.

Provides compile and compilation-related validation methods.
"""

import os
import subprocess
import tempfile
import shutil
from typing import List, Tuple, Optional
from .errors import ErrorInfo, ValidationResult
from .utils import parse_compile_errors


class CompileValidator:
    """Validator for code compilation."""
    
    NIGHTLY = "nightly"
    COMPILE_TIMEOUT = 600  # seconds - 10 minutes
    
    @staticmethod
    def parse_compile_errors(stderr: str) -> List[ErrorInfo]:
        """
        Parse Rust compiler errors from stderr into structured ErrorInfo objects.
        
        Handles formats like:
            error[E0425]: cannot find value `x` in this scope
             --> src/main.rs:5:21
        
        Returns:
            List of ErrorInfo objects
        """
        return parse_compile_errors(stderr)
    
    @staticmethod
    def try_compile_with_cargo(rs_file: str, example_dir: str, nightly: str = NIGHTLY) -> Tuple[bool, List[ErrorInfo]]:
        """Try compiling a .rs file using the example's Cargo.toml context."""
        td = tempfile.mkdtemp()
        try:
            # Copy Cargo.toml and support files
            for f in ["Cargo.toml", "rust-toolchain"]:
                src = os.path.join(example_dir, f)
                if os.path.exists(src):
                    shutil.copy(src, td)
            
            # Copy c2rust-lib.rs
            lib_rs = os.path.join(example_dir, "c2rust-lib.rs")
            if not os.path.exists(lib_rs):
                with open(os.path.join(td, "c2rust-lib.rs"), "w") as f:
                    f.write("#![allow(dead_code)]\n")
                    f.write("extern crate libc;\n")
                    f.write("pub mod main;\n")
            else:
                shutil.copy(lib_rs, td)
            
            # Copy source file as main.rs
            src_main_rs = os.path.join(td, "main.rs")
            shutil.copy(rs_file, src_main_rs)
            
            if not os.path.exists(src_main_rs):
                return False, []
            
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
                ["cargo", f"+{nightly}", "build", "--manifest-path",
                 os.path.join(td, "Cargo.toml")],
                capture_output=True, text=True, timeout=CompileValidator.COMPILE_TIMEOUT,
                env=compile_env
            )
            
            if result.returncode == 0:
                return True, []
            else:
                errors = CompileValidator.parse_compile_errors(result.stderr)
                return False, errors
        
        except subprocess.TimeoutExpired:
            return False, [ErrorInfo(
                error_type="compile_error",
                message=f"Compilation timeout (>{CompileValidator.COMPILE_TIMEOUT}s)",
                details="Compilation exceeded timeout"
            )]
        except Exception as e:
            return False, [ErrorInfo(
                error_type="compile_error",
                message=str(e),
                details=str(e)[:300]
            )]
        finally:
            shutil.rmtree(td, ignore_errors=True)
    
    @staticmethod
    def try_compile_standalone(rs_file: str, example_dir: Optional[str] = None, nightly: str = NIGHTLY) -> Tuple[bool, List[ErrorInfo]]:
        """Try compiling a standalone .rs file."""
        td = tempfile.mkdtemp()
        try:
            compile_env = {**os.environ, "RUSTFLAGS": "-Awarnings"}
            compile_env["CARGO_INCREMENTAL"] = "1"
            
            result = subprocess.run(
                ["rustc", "--edition", "2021", rs_file,
                 "-o", os.path.join(td, "out")],
                capture_output=True, text=True, timeout=CompileValidator.COMPILE_TIMEOUT,
                env=compile_env
            )
            
            if result.returncode == 0:
                return True, []
            
            # If standalone fails and we have example_dir, try cargo
            if example_dir and os.path.exists(example_dir):
                return CompileValidator.try_compile_with_cargo(rs_file, example_dir, nightly)
            
            errors = CompileValidator.parse_compile_errors(result.stderr)
            if not errors:
                errors = [ErrorInfo(
                    error_type="compile_error",
                    message="Compilation failed",
                    details=result.stderr[:500]
                )]
            return False, errors
        
        except subprocess.TimeoutExpired:
            return False, [ErrorInfo(
                error_type="compile_error",
                message=f"Compilation timeout (>{CompileValidator.COMPILE_TIMEOUT}s)",
                details="Compilation took too long"
            )]
        except Exception as e:
            return False, [ErrorInfo(
                error_type="compile_error",
                message=str(e),
                details=str(e)[:300]
            )]
        finally:
            shutil.rmtree(td, ignore_errors=True)

    @staticmethod
    def validate_compile(rs_file: str, example_dir: Optional[str] = None, nightly: str = NIGHTLY) -> ValidationResult:
        """Validate that code compiles."""
        success, errors = CompileValidator.try_compile_standalone(rs_file, example_dir, nightly)
        if success:
            return ValidationResult(True, "compile", "Code compiles successfully", errors=[])
        else:
            error_summary = "\n".join([str(e) for e in errors[:3]])
            error_dicts = [e.to_dict() for e in errors]
            return ValidationResult(
                False, "compile", f"Compilation failed",
                details={"errors": error_dicts, "summary": error_summary},
                errors=errors
            )
