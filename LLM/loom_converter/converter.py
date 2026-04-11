"""Core conversion logic for loom testing."""

import re
import textwrap
from typing import Dict, Optional
from .utils import (
    extract_function_body, extract_once_init_closure,
    detect_array_size_and_element_type, find_functions_using_var
)
from .statics import find_all_global_statics, find_once_init_statics
from .state_gen import generate_state_struct, generate_state_initialization
from .functions import update_function_signature, update_function_calls, replace_global_var_access
from .primitives import (
    replace_concurrency_primitives, wrap_unsafe_function_calls,
    wrap_libc_calls, clone_globals_in_loops
)
from .utils import convert_array_to_fixed_array


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
    
    all_global_statics = find_all_global_statics(content)
    once_statics = find_once_init_statics(content)
    once_init_code = {}
    
    for var_name, var_info in once_statics.items():
        init_code = extract_once_init_closure(content, var_name)
        if init_code:
            once_init_code[var_name] = init_code.strip()
    
    enable_sig_update = True
    
    if enable_sig_update and once_statics and once_init_code:
        for var_name, var_info in once_statics.items():
            var_type = var_info['inner_type']
            using_funcs = find_functions_using_var(content, var_name)
            
            extern_c_funcs = []
            regular_funcs = []
            
            for func_info in using_funcs:
                if 'extern "C"' in func_info['full']:
                    extern_c_funcs.append(func_info)
                else:
                    regular_funcs.append(func_info)
            
            for func_info in regular_funcs:
                func_name = func_info['name']
                func_full = func_info['full']
                
                if func_name in ['main', 'main_0', 'test_concurrent_access']:
                    continue
                
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
                
                updated_sig = update_function_signature(func_full, var_name, param_type)
                updated_sig = re.sub(
                    rf'{re.escape(var_name)}\.get\s*\(\s*\)\.unwrap\s*\(\s*\)',
                    var_name,
                    updated_sig
                )
                
                if updated_sig != func_full:
                    content = content.replace(func_info['full'], updated_sig)
                
                content = update_function_calls(content, func_name, var_name)
    
    if enable_sig_update and all_global_statics:
        for var_name, var_info in all_global_statics.items():
            if var_name in once_statics:
                continue
            
            var_type = var_info['type']
            is_mutable_type = 'Mutex' in var_type or 'Arc' in var_type or var_info['is_mut']
            
            if not is_mutable_type:
                continue
            
            using_funcs = find_functions_using_var(content, var_name)
            
            if using_funcs:
                extern_c_funcs = []
                regular_funcs = []
                
                for func_info in using_funcs:
                    if 'extern "C"' in func_info['full']:
                        extern_c_funcs.append(func_info)
                    else:
                        regular_funcs.append(func_info)
                
                for func_info in regular_funcs:
                    func_name = func_info['name']
                    func_full = func_info['full']
                    
                    if func_name in ['main', 'main_0', 'test_concurrent_access']:
                        continue
                    
                    if var_info['is_mut'] and not ('Mutex' in var_type or 'Arc' in var_type):
                        param_type = f"&mut {var_type}"
                    else:
                        param_type = var_type
                    
                    updated_sig = update_function_signature(func_full, var_name, param_type)
                    
                    if updated_sig != func_full:
                        content = content.replace(func_info['full'], updated_sig)
                    
                    content = update_function_calls(content, func_name, var_name)
                
                for func_info in extern_c_funcs:
                    func_name = func_info['name']
                    func_full = func_info['full']
                    
                    if func_info['full'].count('\n') < 2:
                        continue
                    
                    if var_info['is_mut'] and not ('Mutex' in var_type or 'Arc' in var_type):
                        param_type = f"&mut {var_type}"
                    else:
                        param_type = var_type
                    
                    updated_sig = update_function_signature(func_full, var_name, param_type)
                    
                    if updated_sig != func_full:
                        content = content.replace(func_info['full'], updated_sig)
                    
                    content = update_function_calls(content, func_name, var_name)
    
    main_0_body = extract_function_body(content, 'main_0')
    
    if main_0_body:
        main_0_body = re.sub(r'return\s+0\s*;?\s*$', '', main_0_body.strip(), flags=re.MULTILINE)
        main_0_body = re.sub(r';\s*0\s*$', ';', main_0_body.strip(), flags=re.MULTILINE)
        main_0_body = re.sub(r'\n\s*0\s*$', '', main_0_body.strip(), flags=re.MULTILINE)
        
        lines = main_0_body.split('\n')
        result_lines = []
        i = 0
        
        while i < len(lines):
            line = lines[i]
            
            if 'get_or_init' in line:
                start_paren = line.find('(')
                if start_paren == -1:
                    i += 1
                    continue
                
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
                        
                        if char in ('"', "'") and not in_string:
                            in_string = True
                            string_char = char
                        elif char == string_char and in_string:
                            in_string = False
                            string_char = None
                        elif not in_string:
                            if char == '(':
                                paren_depth += 1
                            elif char == ')':
                                paren_depth -= 1
                            elif char == '{':
                                brace_depth += 1
                            elif char == '}':
                                brace_depth -= 1
                        
                        col += 1
                    
                    if paren_depth == 0 and brace_depth == 0:
                        break
                    
                    j += 1
                    col = 0
                
                i = j + 1
                continue
            
            result_lines.append(line)
            i += 1
        
        main_0_body_joined = '\n'.join(result_lines)
        lines_to_strip = main_0_body_joined.split('\n')
        
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
        
        for var_name in once_init_code:
            using_funcs = find_functions_using_var(content, var_name)
            for func_info in using_funcs:
                func_name = func_info['name']
                
                if 'extern "C"' in func_info['full']:
                    continue
                
                main_0_body = re.sub(
                    rf'\b{re.escape(func_name)}\s*\(\s*\)',
                    f'{func_name}(&{var_name})',
                    main_0_body
                )
        
        for var_name in once_init_code:
            content = re.sub(
                rf'{re.escape(var_name)}\.get_or_init\s*\([^;]*\)\s*;',
                '',
                content,
                flags=re.DOTALL
            )
    
    if main_0_body and all_global_statics:
        main_0_body = replace_global_var_access(main_0_body, all_global_statics)
    
    unsafe_funcs = []
    if main_0_body:
        for match in re.finditer(r'unsafe\s+(?:extern\s+"C"\s+)?fn\s+(\w+)', content):
            func_name = match.group(1)
            if func_name not in ['main', 'main_0', 'test_concurrent_access']:
                unsafe_funcs.append(func_name)
        
        if unsafe_funcs:
            main_0_body = wrap_unsafe_function_calls(main_0_body, unsafe_funcs)
        
        main_0_body = wrap_libc_calls(main_0_body)
    
    state_struct_def = ""
    if all_global_statics:
        state_struct_def = generate_state_struct(all_global_statics, once_statics)
    
    wrapper_funcs_def = ""
    if once_statics and main_0_body:
        for var_name, var_info in once_statics.items():
            inner_type = var_info.get('inner_type', '')
            using_funcs = find_functions_using_var(content, var_name)
            
            for func_info in using_funcs:
                func_name = func_info['name']
                
                if 'extern "C"' not in func_info['full']:
                    continue
                
                func_body = func_info['full']
                wrapper = re.sub(
                    r'unsafe\s+extern\s+"C"\s+fn\s+' + re.escape(func_name) + r'\s*\(\s*\)',
                    f'fn {func_name}_loom_wrapper(num_mutex: &loom::sync::Arc<{inner_type}>)',
                    func_body
                )
                
                wrapper = re.sub(
                    rf'{re.escape(var_name)}\.get\s*\(\s*\)\.unwrap\s*\(\s*\)',
                    'num_mutex',
                    wrapper
                )
                
                if wrapper != func_body:
                    wrapper_funcs_def += wrapper + "\n\n"
    
    init_statements = []
    
    if all_global_statics:
        state_init = generate_state_initialization(all_global_statics, once_statics)
        if state_init:
            init_statements.append(state_init)
    
    for var_name in once_init_code:
        init_code = once_init_code[var_name]
        size, element_expr, fmt = detect_array_size_and_element_type(init_code)
        
        if size and element_expr and fmt:
            init_stmt = f'let {var_name} = loom::sync::Arc::new({init_code});'
        else:
            init_code = convert_array_to_fixed_array(init_code, preserve_array=False)
            init_stmt = f'let {var_name} = {init_code};'
        
        init_statements.append(init_stmt)
    
    if main_0_body and wrapper_funcs_def:
        for var_name in once_statics.keys():
            using_funcs = find_functions_using_var(content, var_name)
            for func_info in using_funcs:
                func_name = func_info['name']
                
                if 'extern "C"' not in func_info['full']:
                    continue
                
                main_0_body = re.sub(
                    rf'\b{re.escape(func_name)}\s*\(\s*\)',
                    f'{func_name}_loom_wrapper(&{var_name})',
                    main_0_body
                )
    
    if main_0_body:
        if init_statements:
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
        
        final_replacement = replacement
        prefix = ""
        if state_struct_def:
            prefix += state_struct_def + "\n"
        if wrapper_funcs_def:
            prefix += wrapper_funcs_def + "\n"
        
        if prefix:
            final_replacement = prefix + replacement
        
        content = content[:start_pos] + final_replacement + content[pos:]
    
    for var_name, var_info in once_statics.items():
        original = var_info['full_match']
        pattern = rf'^[ \t]*{re.escape(original)}\s*\n'
        content = re.sub(pattern, '', content, flags=re.MULTILINE)
        
        if var_info['full_match'] in content:
            content = content.replace(original + '\n', '')
            if original in content:
                content = content.replace(original, '')
    
    for var_name, var_info in all_global_statics.items():
        original = var_info['full_match']
        pattern = rf'(?:^[ \t]*#\[[^\]]+\]\s*\n)?[ \t]*{re.escape(original)}\s*\n?'
        content = re.sub(pattern, '', content, flags=re.MULTILINE)
    
    content = re.sub(r'\n\n\n+', '\n\n', content)
    
    content = re.sub(r'\bloom::loom::', 'loom::', content)
    content = re.sub(r'\bstd::loom::', 'loom::', content)
    
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
    
    if all_global_statics:
        test_body_match = re.search(r'fn test_concurrent_access\s*\(\)\s*\{[\s\S]*?loom::model\s*\(\s*\|\|\s*\{([\s\S]*?)\s*\}\);\s*\}', content)
        if test_body_match:
            test_body = test_body_match.group(1)
            cloned_body = clone_globals_in_loops(test_body, all_global_statics)
            content = content[:test_body_match.start(1)] + cloned_body + content[test_body_match.end(1):]
    
    unused_types = ['Once', 'OnceLock', 'Lazy', 'LazyLock']
    for unused_type in unused_types:
        content = re.sub(rf'use loom::sync::{re.escape(unused_type)};\s*\n', '', content)
        content = re.sub(rf'use std::sync::{re.escape(unused_type)};\s*\n', '', content)
    
    content = re.sub(r'(#\[no_mangle\]\s*)+#\[no_mangle\]', r'#[no_mangle]', content)
    content = re.sub(r'\n\n\n+', '\n\n', content)
    
    return content
