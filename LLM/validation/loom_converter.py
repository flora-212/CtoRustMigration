#!/usr/bin/env python3
"""
Compatibility wrapper for loom_converter.

This file maintains backward compatibility with code that imports the old monolithic loom_converter.py.
All functionality has been refactored into the loom_converter package.
"""

import sys
import os

# Add LLM directory to path to enable importing loom_converter package
llm_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if llm_dir not in sys.path:
    sys.path.insert(0, llm_dir)

# Now import from the package (not from this file)
# We need to avoid importing ourselves, so we remove ourselves from sys.modules first
this_file = os.path.abspath(__file__)
_this_module_name = 'loom_converter'

# Temporarily remove this module from sys.modules so the import will find the package
_self = sys.modules.pop(_this_module_name, None)

try:
    # Now import the real loom_converter package
    import loom_converter as _real_loom
    
    # Re-export all public APIs
    convert_file = _real_loom.convert_file
    add_loom_dependency = _real_loom.add_loom_dependency
    convert_main_to_loom_model = _real_loom.convert_main_to_loom_model
    replace_concurrency_primitives = _real_loom.replace_concurrency_primitives
    main = _real_loom.main
finally:
    # Restore ourselves to sys.modules
    if _self is not None:
        sys.modules[_this_module_name] = _self

__all__ = [
    'convert_file',
    'add_loom_dependency', 
    'convert_main_to_loom_model',
    'replace_concurrency_primitives',
    'main',
]

if __name__ == '__main__':
    main()
#!/usr/bin/env python3
"""
Convert a Rust file for loom concurrency testing.

Usage:
    python loom_converter.py <input.rs> <output.rs> [--example-dir DIR]

This tool:
1. Reads a Rust source file (typically final.rs)
2. Replaces std::sync primitives with loom::sync equivalents
3. Replaces std::thread with loom::thread
4. Converts main() function to a loom::model test
"""

import sys
import re
import textwrap
from pathlib import Path
from typing import Dict, Optional

def find_all_global_statics(content: str) -> Dict[str, Dict]:
    """Find all global static variables (including mutable ones).
    
    Returns:
        Dict mapping variable names to their metadata (type, mutability, initial value).
    """
    statics = {}
    
    # Pattern to find static keyword
    pattern = r'((?:#\[[^\]]*\]\s*\n)*)static\s+(mut\s+)?(\w+)\s*:\s*'
    
    for match in re.finditer(pattern, content, re.MULTILINE):
        is_mut = bool(match.group(2))
        var_name = match.group(3)
        
        # Skip if already in statics
        if var_name in statics:
            continue
        
        # Parse type and initialization by finding matching brackets and semicolon
        start_pos = match.end()
        
        # Find where type ends (look for '=')
        type_end = content.find('=', start_pos)
        if type_end == -1:
            continue
        
        var_type = content[start_pos:type_end].strip()
        
        # Find the ending semicolon, accounting for brackets
        init_start = type_end + 1
        bracket_depth = 0
        pos = init_start
        while pos < len(content):
            char = content[pos]
            if char == '[':
                bracket_depth += 1
            elif char == ']':
                bracket_depth -= 1
            elif char == ';' and bracket_depth == 0:
                # Found the ending semicolon
                init_value = content[init_start:pos].strip()
                
                full_match_text = content[match.start():pos + 1]
                
                statics[var_name] = {
                    'name': var_name,
                    'type': var_type,
                    'is_mut': is_mut,
                    'init_value': init_value,
                    'full_match': full_match_text,
                    'indent': '',
                }
                break
            pos += 1
    
    return statics


def generate_state_struct(global_statics: Dict[str, Dict], once_statics: Dict[str, Dict] = None) -> str:
    """Generate a State struct definition from global static variables.
    
    Args:
        global_statics: Dict of global static variables from find_all_global_statics
        once_statics: Dict of OnceLock variables (to skip their duplication in State)
    
    Returns:
        String containing the State struct definition.
    """
    if not global_statics:
        return ""
    
    if once_statics is None:
        once_statics = {}
    
    struct_def = "struct State {\n"
    for var_name, info in global_statics.items():
        # Skip OnceLock statics - they'll be handled separately
        if var_name in once_statics:
            continue
        
        var_type = info['type']
        is_mut = info['is_mut']
        
        # For mutable statics, wrap in Arc<Mutex<T>>
        if is_mut:
            # Remove 'mut' if already in type
            clean_type = var_type
            field_type = f"Arc<Mutex<{clean_type}>>"
        else:
            field_type = var_type
        
        struct_def += f"    {var_name}: {field_type},\n"
    
    struct_def += "}\n"
    return struct_def


def generate_state_initialization(global_statics: Dict[str, Dict], once_statics: Dict[str, Dict] = None) -> str:
    """Generate initialization code for State struct.
    
    Args:
        global_statics: Dict of global static variables
        once_statics: Dict of OnceLock variables (to skip)
    
    Returns:
        String containing initialization code.
    """
    if not global_statics:
        return "let state = Arc::new(State {});"
    
    if once_statics is None:
        once_statics = {}
    
    state_fields = []
    for var_name, info in global_statics.items():
        # Skip OnceLock statics
        if var_name in once_statics:
            continue
        
        init_value = info['init_value']
        is_mut = info['is_mut']
        
        # For mutable statics, wrap in Arc<Mutex>
        if is_mut:
            state_fields.append(f"        {var_name}: loom::sync::Arc::new(loom::sync::Mutex::new({init_value}))")
        else:
            state_fields.append(f"        {var_name}: {init_value}")
    
    if not state_fields:
        return "let state = Arc::new(State {});"
    
    init_code = "let state = loom::sync::Arc::new(State {\n" + ",\n".join(state_fields) + "\n        });"
    return init_code


def find_once_init_statics(content: str) -> Dict[str, Dict]:
    """Find all static variables that use OnceLock, Once, lazy_static, etc.
    
    Returns:
        Dict mapping variable names to their metadata.
    """
    once_init_types = ['OnceLock', 'Once', 'Lazy', 'LazyLock', 'lazy_static']
    
    statics = {}
    
    # Try each type individually for better matching
    for init_type in once_init_types:
        # Pattern: static [mut] NAME: TYPE<...> = TYPE::new();
        pattern = rf'static\s+(?:mut\s+)?(\w+)\s*:\s*{init_type}<(.+?)>\s*=\s*{init_type}::new\s*\(\s*\)\s*;'
        
        for match in re.finditer(pattern, content, re.DOTALL):
            var_name = match.group(1)
            inner_type = match.group(2).strip()
            
            if not inner_type or var_name in statics:
                continue
            
            statics[var_name] = {
                'type': init_type,
                'full_match': match.group(0),
                'inner_type': inner_type,
            }
    
    return statics


def replace_global_var_access(test_body: str, global_statics: Dict[str, Dict]) -> str:
    """Replace direct global variable access with state.var access in test body.
    
    Converts:
        n1[i] -> state.n1.lock().unwrap()[i]
        n1 = x -> *state.n1.lock().unwrap() = x
        
    Processes in order to avoid double-replacement:
    1. First mark state.fieldname patterns to avoid re-replacing them
    2. Use a single pass with careful matching
    """
    result = test_body
    
    # For each global mutable static, replace direct access with state-based access
    for var_name, info in global_statics.items():
        if not info['is_mut']:
            # Skip immutable statics
            continue
        
        # Use a marker to prevent double-replacement: use a unique token
        marker = f"__STATE_{var_name.upper()}_MARKER__"
        
        # Step 1: Temporarily mark where state.var already exists to avoid double-matching
        result = re.sub(
            rf'state\.{re.escape(var_name)}',
            marker,
            result
        )
        
        # Step 2: Replace array/subscript access: var[index]
        array_pattern = rf'\b{re.escape(var_name)}\s*\[([^\]]*)\]'
        array_replacement = rf'state.{var_name}.lock().unwrap()[\1]'
        result = re.sub(array_pattern, array_replacement, result)
        
        # Step 3: Replace assignment operators: var op=
        # Match word boundary, then var name, then operator
        # Use negative lookbehind to avoid matching if already in state. context
        assign_pattern = rf'\b{re.escape(var_name)}\b\s*(?=[+\-*/%]?=)'
        assign_replacement = rf'*state.{var_name}.lock().unwrap()'
        result = re.sub(assign_pattern, assign_replacement, result)
        
        # Step 4: Replace standalone references that aren't part of state.var
        # (already converted back from marker)
        bare_pattern = rf'(?<!\.)\b{re.escape(var_name)}\b(?![.\[\+\-\*/%=])'
        bare_replacement = rf'*state.{var_name}.lock().unwrap()'
        result = re.sub(bare_pattern, bare_replacement, result)
        
        # Step 5: Restore markers back to state.var (they are already in state. form)
        result = result.replace(marker, f'state.{var_name}')
    
    return result


def wrap_unsafe_function_calls(test_body: str, unsafe_funcs: list) -> str:
    """Wrap unsafe function calls in unsafe blocks.
    
    Args:
        test_body: The test body code
        unsafe_funcs: List of unsafe function names to wrap
    
    Returns:
        Modified test body with unsafe calls wrapped
    """
    if not unsafe_funcs:
        return test_body
    
    lines = test_body.split('\n')
    result_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check if line contains a call to any unsafe function
        found_unsafe_call = False
        for func_name in unsafe_funcs:
            if re.search(rf'\b{re.escape(func_name)}\s*\(', line):
                # Check if not already in unsafe block
                if 'unsafe' not in ''.join(lines[max(0, i-3):i]):
                    found_unsafe_call = True
                    indent = len(line) - len(line.lstrip())
                    indent_str = ' ' * indent
                    
                    # Add unsafe block
                    result_lines.append(f'{indent_str}unsafe {{')
                    
                    # Add the function call line with extra indentation
                    result_lines.append(f'{indent_str}    {line.strip()}')
                    
                    result_lines.append(f'{indent_str}}}')
                    break
        
        if not found_unsafe_call:
            result_lines.append(line)
        
        i += 1
    
    return '\n'.join(result_lines)


def wrap_libc_calls(test_body: str) -> str:
    """Remove or simplify libc calls for loom tests.
    
    Removes printf calls (not easily convertible to println in all cases)
    and wraps other libc calls in unsafe blocks.
    
    Args:
        test_body: The test body code
    
    Returns:
        Modified test body with printf removed and other libc calls wrapped
    """
    lines = test_body.split('\n')
    result_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check for libc::printf - remove the entire printf call (may be multiline)
        if 'libc::printf' in line:
            # Skip this line and any continuation lines until we find the closing );
            j = i
            found_closing = False
            
            while j < len(lines) and not found_closing:
                if ');' in lines[j]:
                    found_closing = True
                j += 1
            
            # Skip all the printf lines we found
            i = j
            continue
        
        # For other libc calls, wrap in unsafe
        if 'libc::' in line:
            # Check if not already in unsafe block
            if 'unsafe' not in ''.join(lines[max(0, i-3):i]):
                base_indent = len(line) - len(line.lstrip())
                indent_str = ' ' * base_indent
                
                # Add unsafe block opening
                result_lines.append(f'{indent_str}unsafe {{')
                
                # Add the libc call line (with double indent)
                result_lines.append(' ' * (base_indent + 4) + line.strip())
                
                # Collect subsequent lines until we find the closing paren and semicolon
                j = i + 1
                paren_depth = line.count('(') - line.count(')')
                
                while j < len(lines) and paren_depth > 0:
                    next_line = lines[j]
                    # Keep original indentation but add extra 4 spaces
                    result_lines.append(' ' * (base_indent + 4) + next_line.strip())
                    paren_depth += next_line.count('(') - next_line.count(')')
                    j += 1
                
                # Add unsafe block closing
                result_lines.append(f'{indent_str}}}')
                
                # Skip the lines we already processed
                i = j
                continue
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines)


def clone_globals_in_loops(test_body: str, all_global_statics: Dict[str, Dict]) -> str:
    """Clone variables when used in thread::spawn within loops.
    
    When a variable (especially Vecs with Arc/Mutex) is used in a move closure 
    inside a loop, it needs to be cloned before the spawn to avoid ownership issues.
    """
    lines = test_body.split('\n')
    result_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check if line contains loom::thread::spawn with move
        if 'loom::thread::spawn' in line and 'move ||' in line:
            # Check if we're inside a loop by looking backwards
            in_loop = False
            
            # Calculate depth at current position
            current_depth = 0
            for check_idx in range(i):
                check_line = lines[check_idx]
                current_depth += check_line.count('{') - check_line.count('}')
            
            # Now look backward
            search_depth = current_depth
            for back_idx in range(i - 1, -1, -1):
                back_line = lines[back_idx]
                search_depth -= back_line.count('{') - back_line.count('}')
                
                # If we found a for/while at lower depth, it encloses current line
                if search_depth < current_depth and re.search(r'\b(for|while)\b', back_line):
                    in_loop = True
                    break
            
            if in_loop:
                # Find which variables are used in the closure
                closure_vars = set()
                
                # Scan forward from spawn to collect closure body
                scan_idx = i
                closure_start = False
                closure_brace_depth = 0
                
                while scan_idx < len(lines):
                    scan_line = lines[scan_idx]
                    
                    # Find the opening ||
                    if '||' in scan_line:
                        closure_start = True
                    
                    if closure_start:
                        closure_brace_depth += scan_line.count('{') - scan_line.count('}')
                    
                    # Extract identifiers from this line
                    for match in re.finditer(r'\b([a-zA-Z_][a-zA-Z0-9_]*)\b', scan_line):
                        var_name = match.group(1)
                        # Filter keywords
                        if var_name not in ['move', 'unsafe', 'fn', 'let', 'if', 'else', 'for', 'while', 
                                           'loop', 'return', 'break', 'continue', 'match', 'true', 'false',
                                           'and', 'or', 'unwrap', 'lock', 'clone', 'join', 'push', 'spawn',
                                           'thread', 'loom', 'handle', 'as', 'const', 'mut', 'unsafe', 'f1']:
                            closure_vars.add(var_name)
                    
                    # Stop when closure is complete
                    if closure_start and closure_brace_depth == 0:
                        break
                    
                    scan_idx += 1
                
                # Now check which of these vars are declared as cloneable types
                candidates = []
                for var_name in closure_vars:
                    for back_idx in range(i - 1, max(0, i - 40), -1):
                        back_line = lines[back_idx]
                        # Match "let varname =" patterns
                        if re.search(rf'\blet\s+{re.escape(var_name)}\s*=', back_line):
                            # Check if it's a cloneable type
                            if any(kw in back_line for kw in ['vec!', 'Vec', 'loom::sync::Arc', 'Arc<']):
                                if var_name not in candidates:
                                    candidates.append(var_name)
                            break
                
                if candidates:
                    # Add clone statements before spawn
                    indent = len(line) - len(line.lstrip())
                    indent_str = ' ' * indent
                    
                    for var_name in candidates:
                        clone_var_name = var_name.lower() + '_clone'
                        result_lines.append(f"{indent_str}let {clone_var_name} = {var_name}.clone();")
                
                # Add the spawn and closure, replacing vars with clones
                modified_line = line
                for var_name in candidates:
                    clone_var_name = var_name.lower() + '_clone'
                    modified_line = re.sub(
                        rf'\b{re.escape(var_name)}\b',
                        clone_var_name,
                        modified_line
                    )
                result_lines.append(modified_line)
                
                # Process the rest of the closure
                i += 1
                closure_brace_depth = modified_line.count('{') - modified_line.count('}')
                found_spawn_close = closure_brace_depth == 0
                
                while i < len(lines) and not found_spawn_close:
                    current = lines[i]
                    modified_current = current
                    
                    # Replace vars with clones
                    for var_name in candidates:
                        clone_var_name = var_name.lower() + '_clone'
                        modified_current = re.sub(
                            rf'\b{re.escape(var_name)}\b',
                            clone_var_name,
                            modified_current
                        )
                    
                    result_lines.append(modified_current)
                    
                    closure_brace_depth += current.count('{') - current.count('}')
                    
                    if closure_brace_depth == 0:
                        found_spawn_close = True
                    
                    i += 1
                
                continue
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines)




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


def detect_array_size_and_element_type(init_code: str) -> tuple:
    """Detect array size and element type from initialization code.
    
    Args:
        init_code: Initialization code like "[Arc::new(Mutex::new(0)); 10]" or 
                   "[Arc::new(Mutex::new(0)), Arc::new(Mutex::new(0)), ...]"
    
    Returns:
        (size, element_type, format) where format is 'repeat' or 'list', 
        or (None, None, None) if not a fixed-size array
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
    # Extract inner content
    inner = init_stripped[1:-1].strip()
    
    if not inner:
        return None, None, None
    
    # Check if it looks like a literal array (contains commas or newlines separating items)
    # Try to count the items by careful parsing
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
                # Item separator
                item_str = ''.join(current_item).strip()
                if item_str:
                    items.append(item_str)
                current_item = []
            else:
                current_item.append(char)
        else:
            current_item.append(char)
    
    # Don't forget the last item
    item_str = ''.join(current_item).strip()
    if item_str:
        items.append(item_str)
    
    if len(items) >= 2:
        # It's a literal array with multiple items
        size = str(len(items))
        # Assume all items are similar, use the first as element type
        element_expr = items[0]
        return size, element_expr, 'list'
    
    return None, None, None


def convert_array_to_fixed_array(init_code: str, preserve_array: bool = False) -> str:
    """Convert array initialization, optionally preserving fixed-size array form.
    
    Args:
        init_code: Initialization code
        preserve_array: If True, keep array form; if False, convert to vec![...]
    
    Returns:
        Modified initialization code
    """
    if preserve_array:
        # Keep array as-is
        return init_code
    else:
        # Convert to vec!
        size, element_expr, fmt = detect_array_size_and_element_type(init_code)
        
        if size and element_expr and fmt:
            if fmt == 'repeat':
                # [expr; size] → vec![expr; size]
                init_code = f'vec![{element_expr}; {size}]'
            elif fmt == 'list':
                # [item1, item2, ...] → vec![item1, item2, ...]
                # Reconstruct from the original, replacing [ with vec![
                init_code = init_code.replace('[', 'vec![', 1)
        else:
            # Not recognized as array, check for simpler cases
            if re.match(r'^\s*\[', init_code.strip()):
                init_code_stripped = init_code.strip()
                if init_code_stripped.startswith('[') and ']' in init_code_stripped:
                    if ';' not in init_code_stripped:
                        # Dynamic array [a, b, c] → vec![a, b, c]
                        inner = init_code_stripped[1:-1]
                        init_code = f'vec![{inner}]'
        
        return init_code


def convert_array_to_vec(init_code: str) -> str:
    """Convert array initialization to Vec initialization (for backward compatibility)."""
    return convert_array_to_fixed_array(init_code, preserve_array=False)

def find_functions_using_var(content: str, var_name: str) -> list:
    """Find all functions that reference a given variable name."""
    functions = []
    
    # Pattern to find function definitions
    # Matches attributes (#[...]), then modifiers (unsafe, extern "C", pub), then fn
    func_pattern = r'(?:#\[[^\]]*\]\s*)*(?:unsafe\s+)?(?:extern\s+"C"\s+)?(?:pub\s+)?fn\s+(\w+)\s*\([^)]*\)\s*(?:->.*?)?\s*\{'
    
    for match in re.finditer(func_pattern, content):
        func_name = match.group(1)
        func_start = match.start()
        func_brace_start = match.end() - 1
        
        # Find function body boundaries
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
        
        # Check if the function uses the variable
        if re.search(rf'\b{re.escape(var_name)}\b', func_body):
            functions.append({
                'name': func_name,
                'start': func_start,
                'end': func_end,
                'full': func_body
            })
    
    return functions


def update_function_signature(func_def: str, var_name: str, var_type: str) -> str:
    """Update a function signature to accept a variable as parameter.
    
    Args:
        func_def: The full function definition text
        var_name: The variable name to add as parameter
        var_type: The type of the variable
    """
    # Pattern to match function signature: [modifiers] fn name (params) [-> return_type] {
    sig_pattern = r'(^[^\{]*?fn\s+\w+\s*)\(([^)]*)\)(\s*(?:->.*?)?\s*\{)'
    
    match = re.search(sig_pattern, func_def, re.MULTILINE | re.DOTALL)
    if not match:
        return func_def
    
    prefix = match.group(1)
    params_inner = match.group(2).strip()
    suffix = match.group(3)
    
    # Check if parameter already exists
    if var_name in params_inner:
        return func_def
    
    # Build the new parameter list
    param_list = []
    if params_inner:
        param_list.append(params_inner)
    
    # Add the new parameter - use & for reference types
    # Check if var_type already has & or it's a built-in type
    if var_type.startswith('&') or var_type in ['i32', 'u32', 'i64', 'u64', 'bool']:
        param_type_str = var_type
    else:
        param_type_str = f"&{var_type}"
    
    new_param = f"{var_name}: {param_type_str}"
    param_list.append(new_param)
    
    new_params = ', '.join(param_list)
    
    # Replace the signature
    new_def = re.sub(
        sig_pattern,
        f"{prefix}({new_params}){suffix}",
        func_def,
        count=1,
        flags=re.MULTILINE | re.DOTALL
    )
    
    return new_def


def update_function_calls(content: str, func_name: str, var_name: str) -> str:
    """Update calls to a function to pass the variable as argument.
    
    Converts: func_name() -> func_name(&var_name)
             func_name(arg) -> func_name(arg, &var_name)
    Properly handles nested parentheses in arguments like ptr::null_mut()
    """
    
    # Find all potential function calls
    pattern = rf'\b{re.escape(func_name)}\s*\('
    
    new_content = []
    last_pos = 0
    
    for match in re.finditer(pattern, content):
        # Extract text before this match
        new_content.append(content[last_pos:match.end()])
        
        # Now find the matching closing parenthesis, handling nested parens
        paren_start = match.end() - 1  # Position of the opening paren
        paren_depth = 1
        pos = paren_start + 1
        in_string = False
        string_char = None
        escape_next = False
        
        while pos < len(content) and paren_depth > 0:
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
            elif not in_string:
                if char == '(':
                    paren_depth += 1
                elif char == ')':
                    paren_depth -= 1
            
            pos += 1
        
        # Extract the arguments (between the parens)
        args = content[paren_start + 1:pos - 1].strip()
        
        # Skip if this looks like it might already have the parameter
        if var_name not in args:
            # Add the new argument
            if args:
                new_args = f"{args}, &{var_name}"
            else:
                new_args = f"&{var_name}"
            new_content.append(new_args + ')')
        else:
            # Keep original
            new_content.append(args + ')')
        
        last_pos = pos
    
    new_content.append(content[last_pos:])
    return ''.join(new_content)


def replace_concurrency_primitives(content: str) -> str:
    """Replace std concurrency primitives with loom equivalents.
    
    Args:
        content: The Rust source code
    """
    
    # Direct replacement for standalone tests
    replacements = [
        # Replace use statements first - be specific
        (r'use std::sync::', 'use loom::sync::'),
        (r'use std::thread;', 'use loom::thread;'),
        (r'use std::thread;', 'use loom::thread;'),
        
        # Replace fully qualified paths in code (before shorter patterns)
        (r'\bstd::sync::(?=Arc|Mutex|RwLock|Condvar|Once|Barrier)', 'loom::sync::'),
        (r'\bstd::thread::(?=spawn|current)', 'loom::thread::'),
        
        # Handle bare thread::spawn (when thread is imported)
        (r'\bthread::spawn\b', 'loom::thread::spawn'),
        
        # Handle Arc and Mutex constructors - only when not already prefixed
        (r'(?<!loom::sync::)(?<!std::sync::)(?<![a-zA-Z_:])Arc::new\b', 'loom::sync::Arc::new'),
        (r'(?<!loom::sync::)(?<!std::sync::)(?<![a-zA-Z_:])Mutex::new\b', 'loom::sync::Mutex::new'),
    ]
    
    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content)
    
    return content


def extract_function_body(content: str, func_name: str) -> str:
    """Extract the body of a function using bracket matching with proper string handling."""
    # Find the function declaration
    pattern = f'fn {re.escape(func_name)}\\s*\\([^)]*\\)(?:\\s*->\\s*[^{{]*)?\\s*\\{{'
    match = re.search(pattern, content)
    
    if not match:
        return None
    
    start_pos = match.end() - 1  # Position of the opening brace
    
    # Find matching closing brace with proper string/comment handling
    brace_depth = 1
    pos = start_pos + 1
    in_string = False
    string_char = None
    escape_next = False
    
    while pos < len(content) and brace_depth > 0:
        char = content[pos]
        
        # Handle escape sequences in strings
        if escape_next:
            escape_next = False
            pos += 1
            continue
        
        if char == '\\' and in_string:
            escape_next = True
            pos += 1
            continue
        
        # Handle string delimiters
        if char in ('"', "'") and not in_string:
            in_string = True
            string_char = char
        elif char == string_char and in_string:
            in_string = False
            string_char = None
        # Handle braces only when not in strings
        elif not in_string:
            if char == '{':
                brace_depth += 1
            elif char == '}':
                brace_depth -= 1
        
        pos += 1
    
    if brace_depth != 0:
        return None
    
    # Extract body without braces
    body = content[start_pos + 1:pos - 1]
    return body


def convert_main_to_loom_model(content: str) -> str:
    """Convert main() function to a loom::model test with full OnceLock support.
    
    This does:
    1. Collects all global static variables into a State struct
    2. Detects OnceLock/Once static variables
    3. Extracts initialization code
    4. Updates function signatures to accept State as parameter
    5. Updates function calls to pass parameters
    6. Builds loom::model test with State initialization
    """
    
    # Step 0: Collect all global static variables
    all_global_statics = find_all_global_statics(content)
    
    # Step 1: Detect and extract once-init static variables
    once_statics = find_once_init_statics(content)
    once_init_code = {}
    
    for var_name, var_info in once_statics.items():
        init_code = extract_once_init_closure(content, var_name)
        if init_code:
            once_init_code[var_name] = init_code.strip()
    
    # Step 2: Update function signatures and calls for OnceLock variables
    # Enabled to pass NUM_MUTEX as parameter to functions like f1()
    enable_sig_update = True
    
    if enable_sig_update and once_statics and once_init_code:
        for var_name, var_info in once_statics.items():
            var_type = var_info['inner_type']
            
            # Find all functions using this variable
            using_funcs = find_functions_using_var(content, var_name)
            
            # Collect extern "C" functions and regular functions separately
            extern_c_funcs = []
            regular_funcs = []
            
            for func_info in using_funcs:
                if 'extern "C"' in func_info['full']:
                    extern_c_funcs.append(func_info)
                else:
                    regular_funcs.append(func_info)
            
            # Update regular functions to accept the variable as parameter
            for func_info in regular_funcs:
                func_name = func_info['name']
                func_full = func_info['full']
                
                # Skip main and main_0 - we'll handle them separately
                if func_name in ['main', 'main_0', 'test_concurrent_access']:
                    continue
                
                # Determine the parameter type
                clean_type = var_type
                if ';' in clean_type:
                    clean_type = clean_type.split(';')[0].strip()
                
                if 'Vec' in clean_type:
                    param_type = clean_type
                elif '[' in clean_type and 'Arc' in clean_type:
                    inner = clean_type.strip('[]')
                    param_type = f"Vec<{inner}>"
                else:
                    param_type = clean_type
                
                # Update this function's signature
                updated_sig = update_function_signature(func_full, var_name, param_type)
                
                # Replace .get().unwrap() calls with the variable name
                updated_sig = re.sub(
                    rf'{re.escape(var_name)}\.get\s*\(\s*\)\.unwrap\s*\(\s*\)',
                    var_name,
                    updated_sig
                )
                
                # Replace in content
                if updated_sig != func_full:
                    content = content.replace(func_info['full'], updated_sig)
                
                # Update all calls to this function
                content = update_function_calls(content, func_name, var_name)
            
            # For extern "C" functions, directly replace with parameter-accepting versions
            for func_info in extern_c_funcs:
                func_name = func_info['name']
                
                # Skip if function body looks empty/minimal
    
    # Step 2b: Also update functions that use global mutable static variables
    # For variables that will be wrapped in State struct
    if enable_sig_update and all_global_statics:
        for var_name, var_info in all_global_statics.items():
            # Skip OnceLock variables (already handled above)
            if var_name in once_statics:
                continue
            
            # Include both mutable statics and Mutex/Arc types (interior mutable)
            var_type = var_info['type']
            is_mutable_type = 'Mutex' in var_type or 'Arc' in var_type or var_info['is_mut']
            
            if not is_mutable_type:
                continue
            
            # Find all functions using this variable
            using_funcs = find_functions_using_var(content, var_name)
            
            if using_funcs:
                # Collect extern "C" functions that use this variable
                extern_c_funcs = []
                regular_funcs = []
                
                for func_info in using_funcs:
                    if 'extern "C"' in func_info['full']:
                        extern_c_funcs.append(func_info)
                    else:
                        regular_funcs.append(func_info)
                
                # Update regular functions to accept the variable as parameter
                for func_info in regular_funcs:
                    func_name = func_info['name']
                    func_full = func_info['full']
                    
                    # Skip main and main_0
                    if func_name in ['main', 'main_0', 'test_concurrent_access']:
                        continue
                    
                    # Build parameter type
                    if var_info['is_mut'] and not ('Mutex' in var_type or 'Arc' in var_type):
                        # Mutable primitive - wrap in Arc<Mutex<T>>
                        param_type = f"&Arc<Mutex<{var_type}>>"
                    else:
                        # Already Mutex or Arc, just pass reference
                        param_type = f"&{var_type}"
                    
                    # Update this function's signature
                    updated_sig = update_function_signature(func_full, var_name, param_type)
                    
                    # Replace in content
                    if updated_sig != func_full:
                        content = content.replace(func_info['full'], updated_sig)
                    
                    # Update all calls to this function
                    content = update_function_calls(content, func_name, var_name)
                
                # Update extern "C" functions that use this variable
                for func_info in extern_c_funcs:
                    func_name = func_info['name']
                    func_full = func_info['full']
                    
                    # Skip if function body looks empty/minimal
                    if func_info['full'].count('\n') < 2:
                        continue
                    
                    # Build parameter type
                    if var_info['is_mut'] and not ('Mutex' in var_type or 'Arc' in var_type):
                        # Mutable primitive - wrap in Arc<Mutex<T>>
                        param_type = f"&Arc<Mutex<{var_type}>>"
                    else:
                        # Already Mutex or Arc, just pass reference
                        param_type = f"&{var_type}"
                    
                    # Update this function's signature
                    updated_sig = update_function_signature(func_full, var_name, param_type)
                    
                    # Replace in content
                    if updated_sig != func_full:
                        content = content.replace(func_info['full'], updated_sig)
                    
                    # Update all calls to this function
                    content = update_function_calls(content, func_name, var_name)
        
        # Handle call chains: propagate parameter requirements through function calls
        # For functions that call functions that need parameters  
        max_iterations = 5
        for iteration in range(max_iterations):
            changes_made = False
            
            for var_name, var_info in all_global_statics.items():
                if var_name in once_statics:
                    continue
                
                var_type = var_info['type']
                is_mutable_type = 'Mutex' in var_type or 'Arc' in var_type or var_info['is_mut']
                
                if not is_mutable_type:
                    continue
                
                # Find functions that have this parameter (already updated)
                funcs_with_param = find_functions_using_var(content, var_name)
                
                # For each function with this parameter
                for func_with_param in funcs_with_param:
                    func_name_with_param = func_with_param['name']
                    # Check if this func actually has the parameter in its signature (not just in calls)
                    func_sig_pattern = r'(?:^|\n)[ \t]*(?:#\[[^\]]*\]\s*)*(?:unsafe\s+)?(?:extern\s+"C"\s+)?(?:pub\s+)?fn\s+' + re.escape(func_name_with_param) + r'\s*\(([^)]*)\)'
                    sig_match = re.search(func_sig_pattern, content)
                    
                    if not sig_match:
                        continue
                    
                    sig_params = sig_match.group(1)
                    # Only consider this function if it has the parameter in its signature
                    if var_name not in sig_params:
                        continue
                    
                    # Find functions that call this function
                    func_pattern = r'(?:^|\n)([ \t]*(?:#\[[^\]]*\]\s*)*)(?:unsafe\s+)?(?:extern\s+"C"\s+)?(?:pub\s+)?fn\s+(\w+)\s*\('
                    
                    for func_match in re.finditer(func_pattern, content, re.MULTILINE):
                        potential_caller_name = func_match.group(2)
                        
                        # Skip if it's the same function
                        if potential_caller_name == func_name_with_param:
                            continue
                        
                        # Skip main functions - they shouldn't have parameters added
                        if potential_caller_name in ['main', 'main_0', 'test_concurrent_access']:
                            continue
                        
                        # Check if this calling function has the parameter in its signature
                        caller_sig_pattern = r'(?:^|\n)[ \t]*(?:#\[[^\]]*\]\s*)*(?:unsafe\s+)?(?:extern\s+"C"\s+)?(?:pub\s+)?fn\s+' + re.escape(potential_caller_name) + r'\s*\(([^)]*)\)'
                        caller_sig_match = re.search(caller_sig_pattern, content)
                        
                        if not caller_sig_match:
                            continue
                        
                        caller_sig_params = caller_sig_match.group(1)
                        
                        # Skip if caller already has this parameter
                        if var_name in caller_sig_params:
                            continue
                        
                        # Extract function body
                        func_body = extract_function_body(content, potential_caller_name)
                        
                        # Check if this function calls the function_with_param
                        if func_body and re.search(rf'\b{re.escape(func_name_with_param)}\s*\(', func_body):
                            # This function calls a function that needs VAR_NAME
                            # Add the parameter to the caller
                            func_def_pattern = r'(?:^|\n)([ \t]*(?:#\[[^\]]*\]\s*)*)(?:unsafe\s+)?(?:extern\s+"C"\s+)?(?:pub\s+)?fn\s+' + re.escape(potential_caller_name) + r'\s*\([^)]*\)'
                            def_match = re.search(func_def_pattern, content)
                            
                            if def_match:
                                old_def = def_match.group(0)
                                # Find the last ) for parameter addition
                                paren_pos = old_def.rfind(')')
                                if paren_pos != -1:
                                    params = old_def[old_def.find('(')+1:paren_pos].strip()
                                    
                                    if params and not params.endswith(','):
                                        new_param = f", {var_name}: &{var_type}"
                                    else:
                                        new_param = f"{var_name}: &{var_type}"
                                    
                                    new_def = old_def[:paren_pos] + new_param + old_def[paren_pos:]
                                    
                                    # Make sure we're only replacing this one occurrence
                                    if new_def != old_def:
                                        content = content.replace(old_def, new_def, 1)
                                        changes_made = True
                                        
                                        # Now update calls from this function
                                        content = update_function_calls(content, potential_caller_name, var_name)
            
            if not changes_made:
                break  # No more changes, exit the iteration loop
    
    # Step 3: Extract main_0 body and clean up initialization
    main_0_body = extract_function_body(content, 'main_0')
    
    # Remove the return statement if found
    if main_0_body:
        main_0_body = re.sub(r'return\s+0\s*;?\s*$', '', main_0_body.strip(), flags=re.MULTILINE)
        main_0_body = re.sub(r';\s*0\s*$', ';', main_0_body.strip(), flags=re.MULTILINE)
        main_0_body = re.sub(r'\n\s*0\s*$', '', main_0_body.strip(), flags=re.MULTILINE)
        
        # Remove or replace get_or_init calls using bracket matching
        # This handles multi-line closures properly
        lines = main_0_body.split('\n')
        result_lines = []
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Check if line contains get_or_init
            if 'get_or_init' in line:
                # Find where the opening paren is
                start_paren = line.find('(')
                if start_paren == -1:
                    i += 1
                    continue
                
                # Track parentheses/braces to find the matching close
                paren_depth = 1
                brace_depth = 0
                j = i
                col = start_paren + 1
                in_string = False
                string_char = None
                
                while j < len(lines) and (paren_depth > 0 or brace_depth > 0):
                    curr_line = lines[j]
                    while col < len(curr_line):
                        char = curr_line[col]
                        
                        if not in_string:
                            if char in ('"', "'"):
                                in_string = True
                                string_char = char
                            elif char == '(':
                                paren_depth += 1
                            elif char == ')':
                                paren_depth -= 1
                            elif char == '{':
                                brace_depth += 1
                            elif char == '}':
                                brace_depth -= 1
                        else:
                            if char == string_char and (col == 0 or curr_line[col-1] != '\\'):
                                in_string = False
                        
                        col += 1
                        if paren_depth == 0 and brace_depth == 0:
                            break
                    
                    if paren_depth == 0 and brace_depth == 0:
                        break
                    
                    j += 1
                    col = 0
                
                # Skip all these lines (they're the get_or_init call)
                i = j + 1
                continue
            
            result_lines.append(line)
            i += 1
        
        # Join lines and remove only leading/trailing empty lines, not indentation
        main_0_body_joined = '\n'.join(result_lines)
        
        # Remove leading/trailing empty lines but preserve indentation of first/last non-empty lines
        lines_to_strip = main_0_body_joined.split('\n')
        
        # Find first and last non-empty lines
        first_non_empty_idx = None
        last_non_empty_idx = None
        for i, line in enumerate(lines_to_strip):
            if line.strip():
                if first_non_empty_idx is None:
                    first_non_empty_idx = i
                last_non_empty_idx = i
        
        if first_non_empty_idx is not None:
            main_0_body = '\n'.join(lines_to_strip[first_non_empty_idx:last_non_empty_idx + 1])
        else:
            main_0_body = ''
        
        # Update all function calls in test to pass the OnceLock variable
        # BUT DON'T try to pass parameters to extern "C" functions (they can't change signature)
        for var_name in once_init_code:
            # Find all functions using this variable
            using_funcs = find_functions_using_var(content, var_name)
            for func_info in using_funcs:
                func_name = func_info['name']
                
                # Skip extern "C" functions - they can't change signature
                if 'extern "C"' in func_info['full']:
                    continue
                
                # Replace func_name() with func_name(&var_name)
                main_0_body = re.sub(
                    rf'\b{re.escape(func_name)}\s*\(\s*\)',
                    f'{func_name}(&{var_name})',
                    main_0_body
                )
        
        # Also remove any get_or_init calls from the global content
        # so that when we extract main_0 again for test creation, it's clean
        for var_name in once_init_code:
            # Remove get_or_init pattern from content as well
            content = re.sub(
                rf'{re.escape(var_name)}\.get_or_init\s*\([^;]*\)\s*;',
                '',
                content,
                flags=re.DOTALL
            )
    
    # Step 4a: Replace global variable access with state.var in test body
    if main_0_body and all_global_statics:
        main_0_body = replace_global_var_access(main_0_body, all_global_statics)
    
    # Step 4b: Collect all unsafe functions and wrap their calls
    unsafe_funcs = []
    if main_0_body:
        # Find all unsafe functions (except main_0, main, test_concurrent_access)
        for match in re.finditer(r'unsafe\s+(?:extern\s+"C"\s+)?fn\s+(\w+)', content):
            func_name = match.group(1)
            if func_name not in ['main', 'main_0', 'test_concurrent_access']:
                unsafe_funcs.append(func_name)
        
        # Wrap unsafe function calls
        if unsafe_funcs:
            main_0_body = wrap_unsafe_function_calls(main_0_body, unsafe_funcs)
        
        # Wrap libc calls in unsafe blocks
        main_0_body = wrap_libc_calls(main_0_body)
    
    # Step 4c: Generate State struct for all global variables
    state_struct_def = ""
    if all_global_statics:
        state_struct_def = generate_state_struct(all_global_statics, once_statics)
    
    # Step 4d: Generate Rust wrappers for extern "C" functions that use OnceLock variables
    # These wrappers replace the extern "C" versions in loom tests
    wrapper_funcs_def = ""
    if once_statics and main_0_body:
        for var_name, var_info in once_statics.items():
            # Get the actual inner type from OnceLock<InnerType>
            inner_type = var_info.get('inner_type', '')
            
            using_funcs = find_functions_using_var(content, var_name)
            for func_info in using_funcs:
                func_name = func_info['name']
                
                # Only create wrappers for extern "C" functions
                if 'extern "C"' not in func_info['full']:
                    continue
                
                # Extract the function body, removing extern "C" and unsafe
                func_body = func_info['full']
                
                # Remove extern "C" and modify to be a regular Rust function that takes the parameter
                # Use the actual inner type instead of placeholder
                wrapper = re.sub(
                    r'unsafe\s+extern\s+"C"\s+fn\s+' + re.escape(func_name) + r'\s*\(\s*\)',
                    f'fn {func_name}_loom_wrapper(num_mutex: &loom::sync::Arc<{inner_type}>)',
                    func_body
                )
                
                # Now replace all references to NUM_MUTEX.get().unwrap() with the parameter
                # This is tricky because we don't know the exact type, but we can make it generic
                wrapper = re.sub(
                    rf'{re.escape(var_name)}\.get\s*\(\s*\)\.unwrap\s*\(\s*\)',
                    'num_mutex',
                    wrapper
                )
                
                if wrapper != func_body:
                    wrapper_funcs_def += wrapper + "\n\n"
    
    # Step 5: Build loom::model test with variable initialization
    # Generate initialization code for once-init variables and global statics
    init_statements = []
    
    # First, handle global static variables
    if all_global_statics:
        # Generate state initialization using helper function
        state_init = generate_state_initialization(all_global_statics, once_statics)
        if state_init:
            init_statements.append(state_init)
    
    # Then handle OnceLock variables (will be passed to functions)
    for var_name in once_init_code:
        init_code = once_init_code[var_name]
        # Keep arrays as fixed-size arrays, wrapped in Arc to avoid drop-related deadlocks
        # Supports both [expr; size] and [item1, item2, ...] formats
        size, element_expr, fmt = detect_array_size_and_element_type(init_code)
        
        if size and element_expr and fmt:
            # This is a fixed-size array - wrap in Arc to share across threads
            init_stmt = f'let {var_name} = loom::sync::Arc::new({init_code});'
        else:
            # Not a fixed-size array, fall back to vec conversion
            init_code = convert_array_to_fixed_array(init_code, preserve_array=False)
            init_stmt = f'let {var_name} = {init_code};'
        
        init_statements.append(init_stmt)
    
    # Replace calls to extern "C" functions with their loom wrapper versions
    if main_0_body and wrapper_funcs_def:
        for var_name in once_statics.keys():
            using_funcs = find_functions_using_var(content, var_name)
            for func_info in using_funcs:
                func_name = func_info['name']
                
                # Only replace for extern "C" functions
                if 'extern "C"' not in func_info['full']:
                    continue
                
                # Replace func_name() with func_name_loom_wrapper(&var_name)
                main_0_body = re.sub(
                    rf'\b{re.escape(func_name)}\s*\(\s*\)',
                    f'{func_name}_loom_wrapper(&{var_name})',
                    main_0_body
                )
    
    if main_0_body:
        # Build the replacement with initialization at the top of loom::model
        # Just use the main_0_body as-is, adding proper indentation
        
        if init_statements:
            # Indent both init statements and main_0_body
            indented_body = textwrap.indent(main_0_body, '        ')
            init_block = '\n        '.join(init_statements)
            replacement = f"""#[test]
fn test_concurrent_access() {{
    loom::model(|| {{
        {init_block}
{indented_body}
    }});
}}"""
        else:
            replacement = f"""#[test]
fn test_concurrent_access() {{
    loom::model(|| {{
{textwrap.indent(main_0_body, '        ')}
    }});
}}"""
    else:
        # Fallback: simpler version without OnceLock variables
        main_body = extract_function_body(content, 'main')
        if main_body:
            main_body = re.sub(r'unsafe\s*\{\s*std::process::exit\([^)]*\)\s*\}', '', main_body)
            replacement = f"""#[test]
fn test_concurrent_access() {{
    loom::model(|| {{
{textwrap.indent(main_body.strip(), '        ')}
    }});
}}"""
        else:
            replacement = """#[test]
fn test_concurrent_access() {
    loom::model(|| {
        // Test logic here
    });
}"""
    
    # Step 6: Replace the main function with the test
    # First try to find unsafe fn main_0, if not found, try fn main
    main_start_patterns = [
        r'(?:pub\s+)?unsafe\s+fn\s+main_0\s*\(\s*\)\s*(?:->\s*\w+\s*)?',
        r'(?:pub\s+)?fn\s+main\s*\(\s*\)\s*\{'
    ]
    
    match = None
    for pattern in main_start_patterns:
        match = re.search(pattern, content)
        if match:
            break
    
    if match:
        start_pos = match.start()
        brace_start = match.end() - 1
        
        # Find matching closing brace with string handling
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
        
        # Inject State struct definition and wrapper functions before the test function
        final_replacement = replacement
        prefix = ""
        if state_struct_def:
            prefix += state_struct_def + "\n"
        if wrapper_funcs_def:
            prefix += wrapper_funcs_def + "\n"
        
        if prefix:
            final_replacement = prefix + replacement
        
        content = content[:start_pos] + final_replacement + content[pos:]
    
    # Step 6b: Post-processing - Replace global variable references in test body with state.variable
    # This must happen right after test generation, before static variable removal
    if '#[test]' in content and 'loom::model' in content:
        test_match = re.search(r'#\[test\]\s*fn\s+test_concurrent_access\s*\(\s*\)\s*\{', content)
        if test_match and all_global_statics:
            # Find the opening brace of loom::model
            loom_start = content.find('loom::model', test_match.start())
            if loom_start != -1:
                # Find the loom::model block
                loom_model_match = re.search(r'loom::model\s*\(\s*\|\|?\s*\{', content[loom_start:])
                if loom_model_match:
                    # Find where loom::model block starts and ends
                    block_start = loom_start + loom_model_match.end()
                    # Find matching closing braces for loom::model
                    brace_depth = 1
                    pos_search = block_start
                    in_string = False
                    string_char = None
                    escape_next = False
                    
                    while pos_search < len(content) and brace_depth > 0:
                        char = content[pos_search]
                        
                        if escape_next:
                            escape_next = False
                            pos_search += 1
                            continue
                        
                        if char == '\\' and in_string:
                            escape_next = True
                            pos_search += 1
                            continue
                        
                        if char in ('"', "'") and not in_string:
                            in_string = True
                            string_char = char
                        elif char == string_char and in_string:
                            in_string = False
                        elif not in_string:
                            if char == '{':
                                brace_depth += 1
                            elif char == '}':
                                brace_depth -= 1
                        
                        pos_search += 1
                    
                    test_body = content[block_start:pos_search-2]  # Exclude closing braces
                    test_body_updated = test_body
                    
                    # Protect all 'let VAR = ...' declarations from being replaced
                    # Replace them with a marker that won't be matched by our substitution regex
                    let_declarations = {}
                    let_marker_id = 0
                    
                    for var_name in all_global_statics.keys():
                        if var_name not in once_statics:
                            # Find 'let var_name =' patterns and protect them
                            pattern = rf'\blet\s+{re.escape(var_name)}\s*='
                            matches = list(re.finditer(pattern, test_body_updated))
                            
                            # Replace from end to start to preserve positions
                            for match in reversed(matches):
                                marker = f"__PROTECTED_LET_{let_marker_id}__"
                                let_declarations[marker] = test_body_updated[match.start():match.end()]
                                test_body_updated = test_body_updated[:match.start()] + marker + ' ' + test_body_updated[match.end():]
                                let_marker_id += 1
                    
                    # Replace references to global mutable statics or Mutex/Arc types with state. prefix
                    # BUT SKIP OnceLock variables - they are independent, not part of State
                    for var_name, var_info in all_global_statics.items():
                        # Skip OnceLock/Once/Lazy variables - they are independent, not part of State
                        if var_name in once_statics:
                            continue
                        
                        var_type = var_info['type']
                        # Include both mutable statics AND Mutex/Arc types (interior mutable)
                        is_mutable = var_info['is_mut'] or 'Mutex' in var_type or 'Arc' in var_type
                        
                        if is_mutable:
                            # Replace &VAR_NAME with &state.VAR_NAME, BUT ONLY if it's not already prefixed
                            # Use negative lookbehind to avoid &state.VAR_NAME or similar
                            test_body_updated = re.sub(rf'(?<![.\w])&{re.escape(var_name)}\b', f'&state.{var_name}', test_body_updated)
                            # Also replace standalone VAR_NAME (not in references) with state.VAR_NAME
                            # BUT ONLY if not already prefixed with state/state_clone/etc
                            test_body_updated = re.sub(rf'(?<![.\w])\b{re.escape(var_name)}\b(?=[\s,\);\]])'  , f'state.{var_name}', test_body_updated)
                    
                    # Restore protected 'let' declarations
                    for marker, declaration in let_declarations.items():
                        test_body_updated = test_body_updated.replace(marker + ' ', declaration)
                    
                    # Replace the test body in content if it changed
                    if test_body_updated != test_body:
                        content = content[:block_start] + test_body_updated + content[pos_search-2:]
    
    # Step 7: Remove all static variable declarations (OnceLock and global)
    # First remove OnceLock static declarations
    for var_name, var_info in once_statics.items():
        original = var_info['full_match']
        
        # Try to remove with leading whitespace and newline
        pattern = rf'^[ \t]*{re.escape(original)}\s*\n'
        content = re.sub(pattern, '', content, flags=re.MULTILINE)
        
        # Fallback: try without newline
        if var_info['full_match'] in content:
            content = content.replace(original + '\n', '')
            if original in content:
                content = content.replace(original, '')
    
    # Then remove all global static variable declarations
    for var_name, var_info in all_global_statics.items():
        original = var_info['full_match']
        
        # Remove including any preceding #[no_mangle] or similar attributes
        # Pattern: optional attribute line(s), then the static declaration
        pattern = rf'(?:^[ \t]*#\[[^\]]+\]\s*\n)?[ \t]*{re.escape(original)}\s*\n?'
        content = re.sub(pattern, '', content, flags=re.MULTILINE)
    
    # Also remove any empty lines left behind
    content = re.sub(r'\n\n\n+', '\n\n', content)
    
    # Step 8: Clean up any double newlines left behind
    
    # Step 8b: Remove extern "C" functions that won't work in loom tests
    # These are typically pthread callbacks that reference OnceLock variables or call parameterized functions
    if enable_sig_update and once_statics:
        # Keep trying to remove extern "C" functions until none are left to remove
        # (use a loop because removing one changes positions for the next)
        while True:
            removed_any = False
            
            # Find all extern "C" functions
            extern_c_pattern = r'(?:^|\n)([ \t]*(?:#\[[\w:]+\]\s*)*)(unsafe\s+)?extern\s+"C"\s+fn\s+(\w+)\s*\([^)]*\)\s*(?:->.*?)?\s*\{'
            
            for match in re.finditer(extern_c_pattern, content, re.MULTILINE):
                func_name = match.group(3)
                func_start = match.start()
                
                # Skip the leading newline if present
                if content[func_start] == '\n':
                    func_start += 1
                
                brace_start = match.end() - 1
                
                # Find the function body
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
                    elif not in_string:
                        if char == '{':
                            brace_depth += 1
                        elif char == '}':
                            brace_depth -= 1
                    
                    pos += 1
                
                func_body = content[brace_start + 1:pos - 1]
                
                # Check if this function should be removed
                should_remove = False
                
                # Check 1: Does it reference any OnceLock variables?
                for var_name in once_statics.keys():
                    if var_name in func_body:
                        should_remove = True
                        break
                
                # Check 2: Does it call any parameterized functions?
                if not should_remove:
                    for var_name in once_statics.keys():
                        # Find functions using this OnceLock variable
                        using_funcs = find_functions_using_var(content, var_name)
                        for func_info in using_funcs:
                            called_func = func_info['name']
                            # Check if this extern "C" calls a parameterized function
                            if f'{called_func}(' in func_body and called_func not in ['main', 'main_0', 'test_concurrent_access']:
                                should_remove = True
                                break
                        if should_remove:
                            break
                
                # Check 3: Is it a pthread-related function that won't be used?
                # These include functions that take c_void pointers (typical pthread callbacks)
                if not should_remove:
                    if 'libc::c_void' in content[match.start():match.end()] or 'c_void' in func_body:
                        # Likely a pthread callback, remove it
                        should_remove = True
                
                if should_remove:
                    # Find the actual start, including attributes
                    remove_start = match.start()
                    
                    # Back up to find preceding #[no_mangle] or similar
                    search_back = remove_start - 1
                    while search_back >= 0 and content[search_back] in (' ', '\t'):
                        search_back -= 1
                    
                    if search_back >= 0 and content[search_back] == '\n':
                        # Check the line before for attributes
                        line_end = search_back
                        line_start = search_back
                        while line_start > 0 and content[line_start - 1] != '\n':
                            line_start -= 1
                        
                        line_content = content[line_start:line_end + 1]
                        if '#[' in line_content:
                            remove_start = line_start
                    
                    # Skip past any closing newline at the end
                    remove_end = pos
                    if remove_end < len(content) and content[remove_end] == '\n':
                        remove_end += 1
                    
                    # Remove the function
                    content = content[:remove_start] + content[remove_end:]
                    removed_any = True
                    break  # Restart search to avoid position issues
            
            if not removed_any:
                break  # No more functions to remove
    
    # Fix any double loom:: prefixes from previous transformations
    content = re.sub(r'\bloom::loom::', 'loom::', content)
    content = re.sub(r'\bstd::loom::', 'loom::', content)
    
    # Step 9: Update thread::spawn calls with move keyword as needed
    content = re.sub(
        r'\bloom::thread::spawn\s*\(\s*\|\|',
        'loom::thread::spawn(move ||',
        content
    )
    content = re.sub(
        r'\bthread::spawn\s*\(\s*\|\|',
        'loom::thread::spawn(move ||',
        content
    )
    
    # Step 9a: Clone global variables in loops with thread::spawn
    # (Done after Step 9 so that 'move' keyword is already added)
    if all_global_statics:
        # Extract the test body from content
        test_body_match = re.search(r'fn test_concurrent_access\s*\(\)\s*\{[\s\S]*?loom::model\s*\(\s*\|\|\s*\{([\s\S]*?)\s*\}\);\s*\}', content)
        if test_body_match:
            test_body = test_body_match.group(1)
            cloned_body = clone_globals_in_loops(test_body, all_global_statics)
            # Replace the test body in content
            content = content[:test_body_match.start(1)] + cloned_body + content[test_body_match.end(1):]
    
    # Step 10: Remove unused imports (Once, OnceLock, etc.)
    # Remove complete import lines for types that are no longer used
    unused_types = ['Once', 'OnceLock', 'Lazy', 'LazyLock']
    for unused_type in unused_types:
        # Remove lines like: use loom::sync::Once;
        content = re.sub(rf'use loom::sync::{re.escape(unused_type)};\s*\n', '', content)
        content = re.sub(rf'use std::sync::{re.escape(unused_type)};\s*\n', '', content)
    
    # Remove duplicate #[no_mangle] attributes
    content = re.sub(r'(#\[no_mangle\]\s*)+#\[no_mangle\]', r'#[no_mangle]', content)
    
    # Also remove any empty lines left behind by import removal
    content = re.sub(r'\n\n\n+', '\n\n', content)
    
    return content


def add_loom_dependency(cargo_toml_path: Path) -> None:
    """Add loom dependency to Cargo.toml if not already present."""
    
    if not cargo_toml_path.exists():
        print(f"Warning: {cargo_toml_path} not found")
        return
    
    content = cargo_toml_path.read_text()
    
    # Check if loom is already in the file
    if 'loom' in content:
        print(f"loom already present in {cargo_toml_path}")
        return
    
    # Add loom dependency under [target.'cfg(loom)'.dependencies]
    if "[target.'cfg(loom)'.dependencies]" not in content:
        content += '\n[target.\'cfg(loom)\'.dependencies]\nloom = "0.7"\n'
    else:
        # loom section exists, add dependency if needed
        if 'loom =' not in content:
            content = content.replace(
                "[target.'cfg(loom)'.dependencies]",
                "[target.'cfg(loom)'.dependencies]\nloom = \"0.7\""
            )
    
    cargo_toml_path.write_text(content)
    print(f"Added loom dependency to {cargo_toml_path}")


def convert_file(input_path: str, output_path: str, example_dir: str = None, standalone: bool = False) -> None:
    """Convert a Rust file for loom testing.
    
    Args:
        input_path: Path to input final.rs file
        output_path: Path to output file
        example_dir: Optional directory for finding Cargo.toml
        standalone: If True, create standalone loom test; If False, just replace primitives
    """
    
    input_file = Path(input_path)
    output_file = Path(output_path)
    
    if not input_file.exists():
        print(f"Error: Input file {input_file} not found")
        sys.exit(1)
    
    # Read the input file
    content = input_file.read_text()
    
    # Step 1: Replace concurrency primitives
    print("Step 1: Replacing concurrency primitives...")
    content = replace_concurrency_primitives(content)
    
    # Step 2: Convert main() to loom::model test (only if standalone mode)
    if standalone:
        print("Step 2: Converting main() to loom::model test...")
        content = convert_main_to_loom_model(content)
    else:
        print("Step 2: Keeping module structure (for library use)...")
        # Just ensure we have the right imports as comments or keep it as-is
        pass
    
    # Step 3: Add loom test modules if needed
    # Ensure we have proper imports for the test
    if standalone and '#[test]' in content and 'loom::model' in content:
        # Check if we have the necessary imports
        if 'use loom' not in content:
            # Add imports at the top
            import_lines = 'use loom::sync::{Arc, Mutex};\nuse loom::thread;\n\n'
            content = import_lines + content
    
    # Write the output file
    output_file.write_text(content)
    print(f"Converted file written to {output_file}")
    
    # Step 4: Add loom dependency to Cargo.toml if example_dir is provided
    if example_dir:
        cargo_path = Path(example_dir) / "Cargo.toml"
        if not cargo_path.exists():
            # Try parent directory
            cargo_path = Path(example_dir).parent / "Cargo.toml"
        
        if cargo_path.exists():
            add_loom_dependency(cargo_path)
    
    print("\nConversion complete!")
    if standalone:
        print(f"To run loom tests: RUSTFLAGS=\"--cfg loom\" cargo test --release")
    else:
        print(f"Module is ready for use with loom. To create loom tests, use --standalone flag.")


def main():
    if len(sys.argv) < 3:
        print("Usage: python loom_converter.py <input.rs> <output.rs> [--example-dir DIR] [--standalone]")
        print("\nOptions:")
        print("  --example-dir DIR    Path to example directory (for finding Cargo.toml)")
        print("  --standalone         Generate standalone loom test (wraps main() in loom::model)")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2]
    example_dir = None
    standalone = False
    
    # Parse optional arguments
    i = 3
    while i < len(sys.argv):
        if sys.argv[i] == '--example-dir':
            if i + 1 < len(sys.argv):
                example_dir = sys.argv[i + 1]
                i += 2
            else:
                print("Error: --example-dir requires an argument")
                sys.exit(1)
        elif sys.argv[i] == '--standalone':
            standalone = True
            i += 1
        else:
            print(f"Unknown option: {sys.argv[i]}")
            sys.exit(1)
    
    convert_file(input_file, output_file, example_dir, standalone)


if __name__ == '__main__':
    main()
