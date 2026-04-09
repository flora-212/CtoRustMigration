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


def replace_concurrency_primitives(content: str, cfg_loom: bool = False) -> str:
    """Replace std concurrency primitives with loom equivalents using AST-like patterns.
    
    Args:
        content: The Rust source code
        cfg_loom: If True, unconditionally use loom types. If False, use cfg(loom) guards.
    """
    
    if cfg_loom:
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
    else:
        # Add cfg(loom) guards for library modules
        # First, replace use statements with cfg-guarded versions
        replacements = [
            # Replace use std::sync with conditional imports
            (r'use std::sync::', 'use std::sync::'),  # Keep std by default
            
            # These will be handled with conditional compilation at top
        ]
    
    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content)
    
    if not cfg_loom:
        # Add conditional compilation block at the top
        cfg_block = '''#[cfg(loom)]
mod loom_primitives {
    pub use loom::sync::{Arc, Mutex};
    pub use loom::thread;
}

#[cfg(not(loom))]
mod loom_primitives {
    pub use std::sync::{Arc, Mutex};
    pub use std::thread;
}

use loom_primitives::*;

'''
        # Remove any existing loom imports (just keep std)
        content = re.sub(r'use loom::[^\n]*\n', '', content)
        
        # Add the cfg block at the beginning (after module-level attributes)
        lines = content.split('\n')
        insert_pos = 0
        for i, line in enumerate(lines):
            if line.strip().startswith('#![') or line.strip() == '':
                insert_pos = i + 1
            else:
                break
        
        lines.insert(insert_pos, cfg_block)
        content = '\n'.join(lines)
    
    return content


def extract_function_body(content: str, func_name: str) -> str:
    """Extract the body of a function using bracket matching."""
    # Find the function declaration
    pattern = f'fn {re.escape(func_name)}\\s*\\([^)]*\\)(?:\\s*->\\s*[^{{]*)?\\s*\\{{'
    match = re.search(pattern, content)
    
    if not match:
        return None
    
    start_pos = match.end() - 1  # Position of the opening brace
    
    # Find matching closing brace
    brace_depth = 1
    pos = start_pos + 1
    while pos < len(content) and brace_depth > 0:
        if content[pos] == '{':
            brace_depth += 1
        elif content[pos] == '}':
            brace_depth -= 1
        pos += 1
    
    if brace_depth != 0:
        return None
    
    # Extract body without braces
    body = content[start_pos + 1:pos - 1]
    return body


def convert_main_to_loom_model(content: str) -> str:
    """Convert main() function to a loom::model test."""
    
    # Try to extract main_0 body first (usually contains the test logic)
    main_0_body = extract_function_body(content, 'main_0')
    
    # Remove the return statement
    if main_0_body:
        # Remove trailing "return 0;" or just "0"
        main_0_body = re.sub(r'return\s+0\s*;?\s*$', '', main_0_body.strip(), flags=re.MULTILINE)
        main_0_body = re.sub(r';\s*0\s*$', ';', main_0_body.strip(), flags=re.MULTILINE)
        main_0_body = re.sub(r'\n\s*0\s*$', '', main_0_body.strip(), flags=re.MULTILINE)
    
    # Find and replace main function
    main_pattern = r'fn main\s*\(\s*\)\s*\{[^{}]*(?:\{[^{}]*\}[^{}]*)*\}'
    
    if main_0_body:
        # Use main_0_body for the test
        replacement = f"""#[test]
fn test_concurrent_access() {{
    loom::model(|| {{
{textwrap.indent(main_0_body, '        ')}
    }});
}}"""
    else:
        # Fallback: try to extract from main directly
        main_body = extract_function_body(content, 'main')
        if main_body:
            # Clean up the exit call
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
    
    content = re.sub(main_pattern, replacement, content, flags=re.DOTALL)
    
    # Remove the main_0 function definition since we've extracted it
    # (optional - you might want to keep it)
    
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
    # For standalone tests, use loom directly. For library modules, use cfg guards
    content = replace_concurrency_primitives(content, cfg_loom=standalone)
    
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
