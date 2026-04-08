#!/usr/bin/env python3
"""
Shared utility functions for validation tools.
"""

import re
from typing import List
from .errors import ErrorInfo


def parse_compile_errors(stderr: str) -> List[ErrorInfo]:
    """
    Parse Rust compiler errors from stderr into structured ErrorInfo objects.
    
    Handles formats like:
        error[E0425]: cannot find value `x` in this scope
         --> src/main.rs:5:21
    
    Args:
        stderr: Error output from Rust compiler
    
    Returns:
        List of ErrorInfo objects
    """
    errors = []
    
    # Split by "error" lines and process each
    lines = stderr.split('\n')
    i = 0
    while i < len(lines):
        line = lines[i]
        
        if line.startswith('error'):
            # Parse error[CODE]: message format
            match = re.match(r'error(?:\[([^\]]+)\])?: (.+)$', line)
            if match:
                error_code = match.group(1) or ""
                message = match.group(2)
                
                # Look for location in next lines
                location = ""
                error_line = -1
                error_col = -1
                details = []
                
                i += 1
                while i < len(lines) and lines[i].startswith(' '):
                    loc_match = re.match(r'\s*-->\s*(.+):(\d+):(\d+)', lines[i])
                    if loc_match:
                        location = loc_match.group(1)
                        error_line = int(loc_match.group(2))
                        error_col = int(loc_match.group(3))
                    else:
                        details.append(lines[i].strip())
                    i += 1
                
                errors.append(ErrorInfo(
                    error_type="compile_error",
                    message=message,
                    location=location,
                    error_code=error_code,
                    line=error_line,
                    column=error_col,
                    details="\n".join(details)
                ))
                continue
        
        i += 1
    
    return errors
