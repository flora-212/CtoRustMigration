"""Functions for replacing concurrency primitives and handling unsafe code."""

import re
from typing import Dict, List, Optional


def replace_concurrency_primitives(content: str) -> str:
    """Replace std concurrency primitives with loom equivalents."""
    replacements = [
        (r'use std::sync::', 'use loom::sync::'),
        (r'use std::thread;', 'use loom::thread;'),
        (r'\bstd::sync::(?=Arc|Mutex|RwLock|Condvar|Once|Barrier)', 'loom::sync::'),
        (r'\bstd::thread::(?=spawn|current)', 'loom::thread::'),
        (r'\bthread::spawn\b', 'loom::thread::spawn'),
        (r'(?<!loom::sync::)(?<!std::sync::)(?<![a-zA-Z_:])Arc::new\b', 'loom::sync::Arc::new'),
        (r'(?<!loom::sync::)(?<!std::sync::)(?<![a-zA-Z_:])Mutex::new\b', 'loom::sync::Mutex::new'),
    ]
    
    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content)
    
    return content


def wrap_unsafe_function_calls(test_body: str, unsafe_funcs: list) -> str:
    """Wrap unsafe function calls in unsafe blocks."""
    if not unsafe_funcs:
        return test_body
    
    lines = test_body.split('\n')
    result_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        found_unsafe_call = False
        
        for func_name in unsafe_funcs:
            if re.search(rf'\b{re.escape(func_name)}\s*\(', line):
                if 'unsafe' not in ''.join(lines[max(0, i-3):i]):
                    found_unsafe_call = True
                    indent = len(line) - len(line.lstrip())
                    indent_str = ' ' * indent
                    
                    result_lines.append(f'{indent_str}unsafe {{')
                    result_lines.append(f'{indent_str}    {line.strip()}')
                    result_lines.append(f'{indent_str}}}')
                    break
        
        if not found_unsafe_call:
            result_lines.append(line)
        
        i += 1
    
    return '\n'.join(result_lines)


def wrap_libc_calls(test_body: str) -> str:
    """Remove or simplify libc calls for loom tests."""
    lines = test_body.split('\n')
    result_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        if 'libc::printf' in line:
            j = i
            found_closing = False
            
            while j < len(lines) and not found_closing:
                if ');' in lines[j]:
                    found_closing = True
                j += 1
            
            i = j
            continue
        
        if 'libc::' in line:
            if 'unsafe' not in ''.join(lines[max(0, i-3):i]):
                base_indent = len(line) - len(line.lstrip())
                indent_str = ' ' * base_indent
                
                result_lines.append(f'{indent_str}unsafe {{')
                result_lines.append(' ' * (base_indent + 4) + line.strip())
                
                j = i + 1
                paren_depth = line.count('(') - line.count(')')
                
                while j < len(lines) and paren_depth > 0:
                    next_line = lines[j]
                    result_lines.append(' ' * (base_indent + 4) + next_line.strip())
                    paren_depth += next_line.count('(') - next_line.count(')')
                    j += 1
                
                result_lines.append(f'{indent_str}}}')
                i = j
                continue
        
        result_lines.append(line)
        i += 1
    
    return '\n'.join(result_lines)


def clone_globals_in_loops(test_body: str, all_global_statics: Dict[str, Dict]) -> str:
    """Clone variables when used in thread::spawn within loops."""
    lines = test_body.split('\n')
    result_lines = []
    i = 0
    clone_index = 1

    def next_clone_name() -> str:
        nonlocal clone_index
        name = f'state{clone_index}'
        clone_index += 1
        return name

    def rewrite_spawn_chunk(chunk_lines: list, clone_name: str) -> list:
        rewritten = []
        for chunk_line in chunk_lines:
            rewritten.append(re.sub(r'\bstate\b', clone_name, chunk_line))
        return rewritten

    while i < len(lines):
        line = lines[i]

        if 'loom::thread::spawn' in line and 'move ||' in line:
            indent = line[:len(line) - len(line.lstrip())]

            clone_name = next_clone_name()
            result_lines.append(indent + f'let {clone_name} = state.clone();')

            chunk_lines = [line]
            brace_depth = line.count('{') - line.count('}')
            j = i + 1

            while j < len(lines) and brace_depth > 0:
                chunk_line = lines[j]
                chunk_lines.append(chunk_line)
                brace_depth += chunk_line.count('{') - chunk_line.count('}')
                j += 1

            result_lines.extend(rewrite_spawn_chunk(chunk_lines, clone_name))
            i = j
            continue

        result_lines.append(line)
        i += 1

    return '\n'.join(result_lines)
