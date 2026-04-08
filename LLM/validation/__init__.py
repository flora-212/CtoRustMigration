#!/usr/bin/env python3
"""
Validation suite for Rust code.

Provides multiple validation tools that can be combined:
  - compile: Check if code compiles (cargo build)
  - clippy: Lint checks (cargo clippy)
  - miri: Undefined behavior detection (cargo miri)
  - loom: Concurrency testing (detects race conditions via permutation testing)
  - safety: Check for unsafe patterns (static analysis)
  - lock_safety: Check lock usage patterns (static analysis)
  - comprehensive: All checks combined (compile, clippy, safety, lock_safety)

Tools can be mixed and matched on the command line:
  python validator.py main.rs compile clippy miri loom

Example usage:
  from validation import CodeValidator, ValidationResult, ErrorInfo
  
  validator = CodeValidator()
  result = validator.validate_compile("main.rs", example_dir="./examples/test")
"""

from .errors import ErrorInfo, ValidationResult
from .tools import ValidationTool
from .core import CodeValidator, validate_file
from .compile import CompileValidator
from .clippy import ClippyValidator
from .miri import MiriValidator
from .loom import LoomValidator
from .safety import SafetyAnalyzer

__all__ = [
    'CodeValidator',
    'ValidationResult',
    'ErrorInfo',
    'ValidationTool',
    'validate_file',
    'CompileValidator',
    'ClippyValidator',
    'MiriValidator',
    'LoomValidator',
    'SafetyAnalyzer',
]

__version__ = '1.0.0'
