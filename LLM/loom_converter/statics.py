"""Functions for handling global static variables."""

import re
from typing import Dict, Optional


def find_all_global_statics(content: str) -> Dict[str, Dict]:
    """Find all global static variables (including mutable ones).
    
    Returns:
        Dict mapping variable names to their metadata (type, mutability, initial value).
    """
    statics = {}
    pattern = r'((?:#\[[^\]]*\]\s*\n)*)static\s+(mut\s+)?(\w+)\s*:\s*'
    
    for match in re.finditer(pattern, content, re.MULTILINE):
        is_mut = bool(match.group(2))
        var_name = match.group(3)
        
        if var_name in statics:
            continue
        
        start_pos = match.end()
        type_end = content.find('=', start_pos)
        
        if type_end == -1:
            continue
        
        var_type = content[start_pos:type_end].strip()
        
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


def find_once_init_statics(content: str) -> Dict[str, Dict]:
    """Find all static variables that use OnceLock, Once, lazy_static, etc.
    
    Returns:
        Dict mapping variable names to their metadata.
    """
    once_init_types = ['OnceLock', 'Once', 'Lazy', 'LazyLock', 'lazy_static']
    statics = {}
    
    for init_type in once_init_types:
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
