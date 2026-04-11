#!/usr/bin/env python3
"""
Loom validation tool.

Provides concurrency testing via loom model checker.
"""

import os
import subprocess
import tempfile
import shutil
import sys
from typing import Optional
from .errors import ErrorInfo, ValidationResult
from .utils import parse_compile_errors

# Import loom_converter - add LLM to path first to find the loom_converter package
llm_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if llm_dir not in sys.path:
    sys.path.insert(0, llm_dir)

try:
    from loom_converter import convert_file as loom_convert_file
except ImportError as e:
    # Fallback for when loom_converter is not in path
    loom_convert_file = None
    import warnings
    warnings.warn(f"Could not import loom_converter: {e}")


class LoomValidator:
    """Validator for concurrency testing with loom."""
    
    NIGHTLY = "nightly"
    LOOM_TIMEOUT = 3600  # 60 minutes
    
    @staticmethod
    def validate_loom(rs_file: str, example_dir: Optional[str] = None, nightly: str = NIGHTLY) -> ValidationResult:
        """
        Validate code with loom (concurrency model checker).
        
        Detects data races and other concurrency issues through state space exploration.
        Reference: https://github.com/tokio-rs/loom
        """
        td = tempfile.mkdtemp()
        try:
            # Copy Cargo.toml
            cargo_toml = os.path.join(example_dir, "Cargo.toml") if example_dir else None
            if cargo_toml and os.path.exists(cargo_toml):
                with open(cargo_toml) as f:
                    content = f.read()
                
                # Inject loom as dev-dependency if not present
                if "loom" not in content:
                    content = content.rstrip() + '\n[dev-dependencies]\nloom = "0.7"\n'
                
                with open(os.path.join(td, "Cargo.toml"), "w") as f:
                    f.write(content)
            
            # Copy rust-toolchain if available
            tc_file = os.path.join(example_dir, "rust-toolchain") if example_dir else None
            if tc_file and os.path.exists(tc_file):
                shutil.copy(tc_file, td)
                with open(tc_file) as f:
                    nightly = f.read().strip()
            
            # Copy c2rust-lib.rs if available
            lib_rs = os.path.join(example_dir, "c2rust-lib.rs") if example_dir else None
            if lib_rs and os.path.exists(lib_rs):
                shutil.copy(lib_rs, td)
            else:
                with open(os.path.join(td, "c2rust-lib.rs"), "w") as f:
                    f.write("#![allow(dead_code)]\n")
                    f.write("extern crate libc;\n")
                    f.write("pub mod main;\n")
            
            # Process rs_file with loom_converter before copying to main.rs
            src_main_rs = os.path.join(td, "main.rs")
            temp_converted_rs = os.path.join(td, "converted.rs")
            
            # Use loom_converter to process the file
            if loom_convert_file:
                try:
                    loom_convert_file(
                        rs_file, 
                        temp_converted_rs, 
                        example_dir=example_dir, 
                        standalone=True
                    )
                    # Use the converted file as main.rs
                    shutil.copy(temp_converted_rs, src_main_rs)
                except Exception as e:
                    # Fallback to original file if conversion fails
                    print(f"Warning: loom_converter failed ({e}), using original file")
                    shutil.copy(rs_file, src_main_rs)
            else:
                # Fallback if loom_converter not available
                shutil.copy(rs_file, src_main_rs)
            
            # Clean build artifacts
            cargo_lock = os.path.join(td, "Cargo.lock")
            if os.path.exists(cargo_lock):
                os.remove(cargo_lock)
            
            target_dir = os.path.join(td, "target")
            if os.path.exists(target_dir):
                shutil.rmtree(target_dir, ignore_errors=True)
            
            compile_env = {**os.environ}
            compile_env["LOOM_CHECKPOINT_INTERVAL"] = "100"
            compile_env["CARGO_INCREMENTAL"] = "1"
            compile_env["RAYON_NUM_THREADS"] = "4"
            compile_env["RUSTFLAGS"] = "--cfg loom"
            
            # Run loom tests
            result = subprocess.run(
                ["cargo", f"+{nightly}", "test", "--manifest-path",
                 os.path.join(td, "Cargo.toml"), "--release"],
                capture_output=True, text=True, timeout=LoomValidator.LOOM_TIMEOUT,
                env=compile_env
            )
            
            if result.returncode == 0:
                return ValidationResult(True, "loom", "Loom analysis passed (no data races detected)", errors=[])
            else:
                errors = parse_compile_errors(result.stderr)
                if not errors:
                    errors = [ErrorInfo(
                        error_type="loom_error",
                        message="Loom detected potential concurrency issue",
                        details=result.stderr[:1000]
                    )]
                
                error_summary = "\n".join([str(e) for e in errors[:3]])
                error_dicts = [e.to_dict() for e in errors]
                return ValidationResult(
                    False, "loom", f"Loom found {len(errors)} issue(s)",
                    details={"errors": error_dicts, "summary": error_summary},
                    errors=errors
                )
        
        except subprocess.TimeoutExpired:
            return ValidationResult(
                False, "loom",
                f"Loom timeout (>{LoomValidator.LOOM_TIMEOUT}s)",
                errors=[ErrorInfo(
                    error_type="loom_error",
                    message=f"Loom timeout (>{LoomValidator.LOOM_TIMEOUT}s)",
                    details="Loom state space exploration took too long (>60 minutes). This suggests complex concurrency patterns that exceed loom's scalability."
                )]
            )
        except Exception as e:
            return ValidationResult(
                False, "loom", str(e),
                errors=[ErrorInfo(
                    error_type="loom_error",
                    message=str(e),
                    details=str(e)[:300]
                )]
            )
        finally:
            shutil.rmtree(td, ignore_errors=True)
