"""
Validation module for Rust code safety and correctness analysis.

Provides tools for:
- Static safety analysis (unsafe blocks, raw pointers, etc.)
- Compilation verification
- Correctness testing with Loom
"""

from .core import CodeValidator
from .safety import SafetyAnalyzer

__all__ = ['CodeValidator', 'SafetyAnalyzer']
