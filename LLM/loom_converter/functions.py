"""Functions for handling function signatures and calls."""

import re
from typing import Dict, Optional


def update_function_signature(func_def: str, var_name: str, var_type: str) -> str:
    """Update a function signature to accept a variable as parameter."""
    sig_pattern = r'(^[^\{]*?fn\s+\w+\s*)\(([^)]*)\)(\s*(?:->.*?)?\s*\{)'
    
    match = re.search(sig_pattern, func_def, re.MULTILINE | re.DOTALL)
    if not match:
        return func_def
    
    prefix = match.group(1)
    params_inner = match.group(2).strip()
    suffix = match.group(3)
    
    if var_name in params_inner:
        return func_def
    
    param_list = []
    if params_inner:
        param_list.append(params_inner)
    
    if var_type.startswith('&') or var_type in ['i32', 'u32', 'i64', 'u64', 'bool']:
        param_type_str = var_type
    else:
        param_type_str = f"&{var_type}"
    
    new_param = f"{var_name}: {param_type_str}"
    param_list.append(new_param)
    
    new_params = ', '.join(param_list)
    
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
    """
    pattern = rf'\b{re.escape(func_name)}\s*\('
    new_content = []
    last_pos = 0
    
    for match in re.finditer(pattern, content):
        new_content.append(content[last_pos:match.end()])
        
        paren_start = match.end() - 1
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
        
        args = content[paren_start + 1:pos - 1].strip()
        
        if var_name not in args:
            if args:
                new_args = f"{args}, &{var_name}"
            else:
                new_args = f"&{var_name}"
            new_content.append(new_args + ')')
        else:
            new_content.append(args + ')')
        
        last_pos = pos
    
    new_content.append(content[last_pos:])
    return ''.join(new_content)


def replace_global_var_access(test_body: str, global_statics: Dict[str, Dict]) -> str:
    """Replace direct global variable access with state.var access in test body.
    
    Converts:
        n1[i] -> state.n1.lock().unwrap()[i]
        n1 = x -> *state.n1.lock().unwrap() = x
    """
    result = test_body
    
    for var_name, info in global_statics.items():
        if not info['is_mut']:
            continue
        
        marker = f"__STATE_{var_name.upper()}_MARKER__"
        
        result = re.sub(
            rf'state\.{re.escape(var_name)}',
            marker,
            result
        )
        
        array_pattern = rf'\b{re.escape(var_name)}\s*\[([^\]]*)\]'
        array_replacement = rf'state.{var_name}.lock().unwrap()[\1]'
        result = re.sub(array_pattern, array_replacement, result)
        
        assign_pattern = rf'\b{re.escape(var_name)}\b\s*(?=[+\-*/%]?=)'
        assign_replacement = rf'*state.{var_name}.lock().unwrap()'
        result = re.sub(assign_pattern, assign_replacement, result)
        
        bare_pattern = rf'(?<!\.)\b{re.escape(var_name)}\b(?![.\[\+\-\*/%=])'
        bare_replacement = rf'*state.{var_name}.lock().unwrap()'
        result = re.sub(bare_pattern, bare_replacement, result)
        
        result = result.replace(marker, f'state.{var_name}')
    
    return result
