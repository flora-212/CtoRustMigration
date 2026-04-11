"""Functions for generating State struct and initialization code."""

from typing import Dict, Optional


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
        if var_name in once_statics:
            continue
        
        var_type = info['type']
        is_mut = info['is_mut']
        
        if is_mut:
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
        if var_name in once_statics:
            continue
        
        init_value = info['init_value']
        is_mut = info['is_mut']
        
        if is_mut:
            state_fields.append(f"        {var_name}: loom::sync::Arc::new(loom::sync::Mutex::new({init_value}))")
        else:
            state_fields.append(f"        {var_name}: {init_value}")
    
    if not state_fields:
        return "let state = Arc::new(State {});"
    
    init_code = "let state = loom::sync::Arc::new(State {\n" + ",\n".join(state_fields) + "\n        });"
    return init_code
