"""Command-line interface and main file conversion functions."""

import sys
from pathlib import Path
from .primitives import replace_concurrency_primitives
from .converter import convert_main_to_loom_model


def add_loom_dependency(cargo_toml_path: Path) -> None:
    """Add loom dependency to Cargo.toml if not already present."""
    if not cargo_toml_path.exists():
        print(f"Warning: {cargo_toml_path} not found")
        return
    
    content = cargo_toml_path.read_text()
    
    if 'loom' in content:
        print(f"loom already present in {cargo_toml_path}")
        return
    
    if "[target.'cfg(loom)'.dependencies]" not in content:
        content += '\n[target.\'cfg(loom)\'.dependencies]\nloom = "0.7"\n'
    else:
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
    
    content = input_file.read_text()
    
    print("Step 1: Replacing concurrency primitives...")
    content = replace_concurrency_primitives(content)
    
    if standalone:
        print("Step 2: Converting main() to loom::model test...")
        content = convert_main_to_loom_model(content)
    else:
        print("Step 2: Keeping module structure (for library use)...")
    
    if standalone and '#[test]' in content and 'loom::model' in content:
        if 'use loom' not in content:
            import_lines = 'use loom::sync::{Arc, Mutex};\nuse loom::thread;\n\n'
            content = import_lines + content
    
    output_file.write_text(content)
    print(f"Converted file written to {output_file}")
    
    if example_dir:
        cargo_path = Path(example_dir) / "Cargo.toml"
        if not cargo_path.exists():
            cargo_path = Path(example_dir).parent / "Cargo.toml"
        
        if cargo_path.exists():
            add_loom_dependency(cargo_path)
    
    print("\nConversion complete!")
    if standalone:
        print(f"To run loom tests: RUSTFLAGS=\"--cfg loom\" cargo test --release")
    else:
        print(f"Module is ready for use with loom. To create loom tests, use --standalone flag.")


def main():
    """Command-line entry point."""
    if len(sys.argv) < 3:
        print("Usage: python -m loom_converter <input.rs> <output.rs> [--example-dir DIR] [--standalone]")
        print("\nOptions:")
        print("  --example-dir DIR    Path to example directory (for finding Cargo.toml)")
        print("  --standalone         Generate standalone loom test (wraps main() in loom::model)")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2]
    example_dir = None
    standalone = False
    
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
