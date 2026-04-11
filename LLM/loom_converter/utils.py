"""Utility functions for loom converter."""

import re
from typing import Optional, Dict, Tuple


def extract_function_body(content: str, func_name: str) -> Optional[str]:
    """Extract the body of a function using bracket matching with proper string handling."""
    pattern = f'fn {re.escape(func_name)}\\s*\\([^)]*\\)(?:\\s*->\\s*[^{{]*)?\\s*\\{{'
    match = re.search(pattern, content)
    
    if not match:
        return None
    
    start_pos = match.end() - 1
    brace_depth = 1
    pos = start_pos + 1
    in_string = False
    string_char = None
    escape_next = False
    
    while pos < len(content) and brace_depth > 0:
        char = content[pos]
        
        if escape_next:
            escape_next = False
            pos += 1
            continue
        
        if char == '\\' and in_string:
            escape_next = True
            pos += 1
            continue
        
        if char in ('"', "'") and not in_string:
            in_string = True
            string_char = char
        elif char == string_char and in_string:
            in_string = False
            string_char = None
        elif not in_string:
            if char == '{':
                brace_depth += 1
            elif char == '}':
                brace_depth -= 1
        
        pos += 1
    
    if brace_depth != 0:
        return None
    
    body = content[start_pos + 1:pos - 1]
    return body


def extract_once_init_closure(content: str, var_name: str) -> Optional[str]:
    """Extract initialization closure from get_or_init/call_once patterns."""
    pattern_getorinit = rf'{re.escape(var_name)}\.get_or_init\s*\(\s*\|\|\s*\{{'
    pattern_callonce = rf'{re.escape(var_name)}\.call_once\s*\(\s*\|\|\s*\{{'
    
    for pattern in [pattern_getorinit, pattern_callonce]:
        match = re.search(pattern, content)
        if match:
            brace_start = match.end() - 1
            brace_depth = 1
            pos = brace_start + 1
            in_string = False
            string_char = None
            escape_next = False
            
            while pos < len(content) and brace_depth > 0:
                char = content[pos]
                
                if escape_next:
                    escape_next = False
                    pos += 1
                    continue
                
                if char == '\\' and in_string:
                    escape_next = True
                    pos += 1
                    continue
                
                if char in ('"', "'") and not in_string:
                    in_string = True
                    string_char = char
                elif char == string_char and in_string:
                    in_string = False
                    string_char = None
                elif not in_string:
                    if char == '{':
                        brace_depth += 1
                    elif char == '}':
                        brace_depth -= 1
                
                pos += 1
            
            if brace_depth == 0:
                closure_body = content[brace_start + 1:pos - 1].strip()
                return closure_body
    
    return None


def detect_array_size_and_element_type(init_code: str) -> Tuple[Optional[str], Optional[str], Optional[str]]:
    """Detect array size and element type from initialization code.
    
    Returns:
        (size, element_type, format) where format is 'repeat' or 'list'
    """
    init_stripped = init_code.strip()
    
    if not init_stripped.startswith('[') or not init_stripped.endswith(']'):
        return None, None, None
    
    # Pattern 1: [expr; size] (repeat format)
    array_repeat_pattern = r'^\[(.+);\s*(\d+)\]$'
    match = re.match(array_repeat_pattern, init_stripped, re.DOTALL)
    
    if match:
        element_expr = match.group(1).strip()
        size = match.group(2)
        return size, element_expr, 'repeat'
    
    # Pattern 2: [item1, item2, ...] (list format)
    inner = init_stripped[1:-1].strip()
    
    if not inner:
        return None, None, None
    
    items = []
    current_item = []
    bracket_depth = 0
    paren_depth = 0
    in_string = False
    string_char = None
    escape_next = False
    
    for char in inner:
        if escape_next:
            current_item.append(char)
            escape_next = False
            continue
        
        if char == '\\' and in_string:
            current_item.append(char)
            escape_next = True
            continue
        
        if char in ('"', "'") and not in_string:
            in_string = True
            string_char = char
            current_item.append(char)
        elif char == string_char and in_string:
            in_string = False
            string_char = None
            current_item.append(char)
        elif not in_string:
            if char in ('[', '{'):
                bracket_depth += 1
                current_item.append(char)
            elif char in (']', '}'):
                bracket_depth -= 1
                current_item.append(char)
            elif char == '(':
                paren_depth += 1
                current_item.append(char)
            elif char == ')':
                paren_depth -= 1
                current_item.append(char)
            elif char == ',' and bracket_depth == 0 and paren_depth == 0:
                item_str = ''.join(current_item).strip()
                if item_str:
                    items.append(item_str)
                current_item = []
            else:
                current_item.append(char)
        else:
            current_item.append(char)
    
    item_str = ''.join(current_item).strip()
    if item_str:
        items.append(item_str)
    
    if len(items) >= 2:
        size = str(len(items))
        element_expr = items[0]
        return size, element_expr, 'list'
    
    return None, None, None


def convert_array_to_fixed_array(init_code: str, preserve_array: bool = False) -> str:
    """Convert array initialization, optionally preserving fixed-size array form."""
    if preserve_array:
        return init_code
    
    size, element_expr, fmt = detect_array_size_and_element_type(init_code)
    
    if size and element_expr and fmt:
        if fmt == 'repeat':
            init_code = f'vec![{element_expr}; {size}]'
        elif fmt == 'list':
            init_code = init_code.replace('[', 'vec![', 1)
    else:
        if re.match(r'^\s*\[', init_code.strip()):
            init_code_stripped = init_code.strip()
            if init_code_stripped.startswith('[') and ']' in init_code_stripped:
                if ';' not in init_code_stripped:
                    init_code = 'vec!' + init_code
    
    return init_code


def find_functions_using_var(content: str, var_name: str) -> list:
    """Find all functions that reference a given variable name."""
    functions = []
    
    func_pattern = r'(?:#\[[^\]]*\]\s*)*(?:unsafe\s+)?(?:extern\s+"C"\s+)?(?:pub\s+)?fn\s+(\w+)\s*\([^)]*\)\s*(?:->.*?)?\s*\{'
    
    for match in re.finditer(func_pattern, content):
        func_name = match.group(1)
        func_start = match.start()
        func_brace_start = match.end() - 1
        
        brace_depth = 1
        pos = func_brace_start + 1
        in_string = False
        string_char = None
        escape_next = False
        
        while pos < len(content) and brace_depth > 0:
            char = content[pos]
            
            if escape_next:
                escape_next = False
                pos += 1
                continue
            
            if char == '\\' and in_string:
                escape_next = True
                pos += 1
                continue
            
            if char in ('"', "'") and not in_string:
                in_string = True
                string_char = char
            elif char == string_char and in_string:
                in_string = False
                string_char = None
            elif not in_string:
                if char == '{':
                    brace_depth += 1
                elif char == '}':
                    brace_depth -= 1
            
            pos += 1
        
        func_end = pos
        func_body = content[func_start:func_end]
        
        if re.search(rf'\b{re.escape(var_name)}\b', func_body):
            functions.append({
                'name': func_name,
                'start': func_start,
                'end': func_end,
                'full': func_body
            })
    
    return functions
