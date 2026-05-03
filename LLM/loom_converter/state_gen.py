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
            vt = var_type.strip()
            # Case A: per-element Mutex array like "[Mutex<i32>; 5]"
            if '[' in vt and 'Mutex' in vt:
                field_type = f"Arc<{vt}>"
            # Case B: plain mutable fixed-size array (keep race semantics with UnsafeCell)
            elif vt.startswith('['):
                field_type = f"Arc<loom::cell::UnsafeCell<{vt}>>"
            # Default: wrap in Arc<Mutex<...>> for other mutable types
            else:
                field_type = f"Arc<Mutex<{vt}>>"
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
        vt = info['type'].strip()

        if is_mut:
            # Per-element Mutex array: keep array of Mutex and wrap in Arc (no outer Mutex)
            if '[' in vt and 'Mutex' in vt:
                state_fields.append(f"        {var_name}: loom::sync::Arc::new({init_value})")
            # Plain mutable fixed-size array: preserve race semantics with UnsafeCell inside Arc
            elif vt.startswith('['):
                state_fields.append(f"        {var_name}: loom::sync::Arc::new(loom::cell::UnsafeCell::new({init_value}))")
            # Default: wrap in Arc<Mutex<...>>
            else:
                state_fields.append(f"        {var_name}: loom::sync::Arc::new(loom::sync::Mutex::new({init_value}))")
        else:
            state_fields.append(f"        {var_name}: {init_value}")

    if not state_fields:
        return "let state = loom::sync::Arc::new(State {});"

    init_code = "let state = loom::sync::Arc::new(State {\n" + ",\n".join(state_fields) + "\n        });"
    return init_code
