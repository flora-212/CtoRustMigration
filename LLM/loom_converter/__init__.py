"""Loom converter - Convert Rust code for loom concurrency testing.

This module provides tools to:
1. Replace std::sync primitives with loom::sync equivalents
2. Convert main() functions to loom::model tests
3. Handle global static variables and function signatures
"""

from .cli import convert_file, add_loom_dependency, main
from .converter import convert_main_to_loom_model
from .primitives import replace_concurrency_primitives

__all__ = [
    'convert_file',
    'add_loom_dependency',
    'convert_main_to_loom_model',
    'replace_concurrency_primitives',
    'main',
]

__version__ = '1.0.0'
