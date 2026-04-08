#!/usr/bin/env python3
"""
Validation tools available in the validation suite.
"""

from enum import Enum


class ValidationTool(Enum):
    """Available validation tools."""
    COMPILE = "compile"
    CLIPPY = "clippy"
    MIRI = "miri"
    LOOM = "loom"
    SAFETY = "safety"
    LOCK_SAFETY = "lock_safety"
    COMPREHENSIVE = "comprehensive"
