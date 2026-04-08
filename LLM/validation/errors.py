#!/usr/bin/env python3
"""
Error information and validation results.

Provides structured error reporting for all validation tools.
"""

from typing import Dict, List, Any


class ErrorInfo:
    """Structured error information."""
    
    def __init__(self, error_type: str, message: str, location: str = "", 
                 error_code: str = "", line: int = -1, column: int = -1, details: str = ""):
        """
        Args:
            error_type: "compile_error", "safety_issue", "lock_safety_issue", etc.
            message: Human-readable error message
            location: File location (e.g., "src/main.rs:5:21")
            error_code: Error code (e.g., "E0425")
            line: Line number (-1 if unknown)
            column: Column number (-1 if unknown)
            details: Full error details or additional context
        """
        self.error_type = error_type
        self.message = message
        self.location = location
        self.error_code = error_code
        self.line = line
        self.column = column
        self.details = details
    
    def to_dict(self):
        """Convert to dictionary for serialization."""
        return {
            "type": self.error_type,
            "code": self.error_code,
            "message": self.message,
            "location": self.location,
            "line": self.line,
            "column": self.column,
            "details": self.details[:500] if self.details else ""  # Truncate for JSON
        }
    
    def __repr__(self):
        loc_str = f" [{self.location}]" if self.location else ""
        code_str = f" ({self.error_code})" if self.error_code else ""
        return f"{self.message}{code_str}{loc_str}"


class ValidationResult:
    """Result of a validation check."""
    
    def __init__(self, passed: bool, category: str, message: str = "", details: Dict = None, errors: List[ErrorInfo] = None):
        self.passed = passed
        self.category = category
        self.message = message
        self.details = details or {}
        self.errors = errors or []  # List of structured ErrorInfo
    
    def __repr__(self):
        status = "✅" if self.passed else "❌"
        return f"{status} [{self.category}] {self.message}"
