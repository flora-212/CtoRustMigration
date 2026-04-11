#!/usr/bin/env python3
"""Test script to verify loom_converter module structure."""

import sys
import os

def test_module_imports():
    """Test all module imports."""
    print("=" * 60)
    print("Testing loom_converter module imports...")
    print("=" * 60)
    
    # Add LLM to path
    llm_dir = os.path.dirname(os.path.abspath(__file__))
    if llm_dir not in sys.path:
        sys.path.insert(0, llm_dir)
    
    try:
        # Test 1: Import main package
        print("\n[1/6] Importing loom_converter package...")
        from loom_converter import (
            convert_file,
            add_loom_dependency,
            convert_main_to_loom_model,
            replace_concurrency_primitives,
            main,
        )
        print("✓ Successfully imported loom_converter package")
        
        # Test 2: Import individual modules
        print("\n[2/6] Importing individual modules...")
        from loom_converter import utils, statics, state_gen, functions, primitives, converter, cli
        print("✓ Successfully imported all submodules")
        
        # Test 3: Test function availability
        print("\n[3/6] Checking function availability...")
        
        functions_to_check = [
            ('utils', ['extract_function_body', 'find_functions_using_var', 'detect_array_size_and_element_type']),
            ('statics', ['find_all_global_statics', 'find_once_init_statics']),
            ('state_gen', ['generate_state_struct', 'generate_state_initialization']),
            ('functions', ['update_function_signature', 'update_function_calls', 'replace_global_var_access']),
            ('primitives', ['replace_concurrency_primitives', 'wrap_unsafe_function_calls', 'wrap_libc_calls']),
            ('converter', ['convert_main_to_loom_model']),
            ('cli', ['convert_file', 'add_loom_dependency', 'main']),
        ]
        
        for module_name, funcs in functions_to_check:
            module = sys.modules[f'loom_converter.{module_name}']
            for func_name in funcs:
                if not hasattr(module, func_name):
                    print(f"✗ Missing function: {module_name}.{func_name}")
                    return False
        
        print("✓ All required functions are available")
        
        # Test 4: Backward compatibility
        print("\n[4/6] Testing backward compatibility wrapper...")
        os.chdir(os.path.join(llm_dir, 'validation'))
        import loom_converter as loom_compat
        
        if not hasattr(loom_compat, 'convert_file'):
            print("✗ Backward compatibility wrapper missing convert_file")
            return False
        
        print("✓ Backward compatibility wrapper works")
        
        # Test 5: Integration with loom.py
        print("\n[5/6] Testing integration with loom.py...")
        from validation.loom import LoomValidator
        print("✓ LoomValidator imports successfully")
        
        # Test 6: Check conversion signature
        print("\n[6/6] Checking function signatures...")
        import inspect
        
        sig = inspect.signature(convert_file)
        params = list(sig.parameters.keys())
        expected = ['input_path', 'output_path', 'example_dir', 'standalone']
        
        if params != expected:
            print(f"✗ convert_file signature mismatch. Expected {expected}, got {params}")
            return False
        
        print("✓ Function signatures are correct")
        
        return True
        
    except Exception as e:
        print(f"\n✗ Error: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    success = test_module_imports()
    
    print("\n" + "=" * 60)
    if success:
        print("✓ All tests passed!")
        print("=" * 60)
        print("\nModule structure is correct and ready for use.")
        return 0
    else:
        print("✗ Some tests failed!")
        print("=" * 60)
        return 1

if __name__ == '__main__':
    sys.exit(main())
