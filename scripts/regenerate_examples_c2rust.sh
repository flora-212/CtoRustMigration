#!/bin/bash

# ════════════════════════════════════════════════════════════════════════════
# Regenerate C2Rust translations for examples and/or examples_negative
# ════════════════════════════════════════════════════════════════════════════
#
# This script will:
#   1. For each directory in examples/ and/or examples_negative/
#   2. Run c2rust transpile on a.c to generate Rust code
#   3. Rename the generated a.rs to main.c2rust.rs (the c2rust translation)
#   4. Copy main.c2rust.rs to main.rs (so pub mod main; in c2rust-lib.rs works)
#   5. Ensure c2rust-lib.rs is properly set up
#
# Usage:
#   ./scripts/regenerate_examples_c2rust.sh                    # Process both folders (default)
#   ./scripts/regenerate_examples_c2rust.sh examples           # Process only examples/
#   ./scripts/regenerate_examples_c2rust.sh examples_negative  # Process only examples_negative/
#   ./scripts/regenerate_examples_c2rust.sh --force            # Ignore existing main.c2rust.rs
#   ./scripts/regenerate_examples_c2rust.sh examples --force   # Process examples/ with --force

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
C2RUST_BIN="${C2RUST_BIN:=/home/guoxy/c2rust/target/release/c2rust}"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'  # No Color

# Parse arguments
FORCE_REGENERATE=false
PROCESS_FOLDERS=()

for arg in "$@"; do
    case "$arg" in
        --force)
            FORCE_REGENERATE=true
            ;;
        examples|examples_negative)
            PROCESS_FOLDERS+=("$arg")
            ;;
        *)
            echo "Unknown argument: $arg"
            echo ""
            echo "Usage:"
            echo "  $0                           # Process both folders (default)"
            echo "  $0 examples                  # Process only examples/"
            echo "  $0 examples_negative         # Process only examples_negative/"
            echo "  $0 --force                   # Regenerate all (ignore existing files)"
            echo "  $0 examples --force          # Process examples/ and regenerate"
            exit 1
            ;;
    esac
done

# Default to both folders if none specified
if [ ${#PROCESS_FOLDERS[@]} -eq 0 ]; then
    PROCESS_FOLDERS=("examples" "examples_negative")
fi

# Check if c2rust is available
if [ ! -x "$C2RUST_BIN" ]; then
    echo -e "${RED}❌ Error: c2rust binary not found at $C2RUST_BIN${NC}"
    echo "   Please set C2RUST_BIN environment variable or install c2rust"
    exit 1
fi

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Regenerating C2Rust Translations                             ║${NC}"
echo -e "${BLUE}║  Output: main.c2rust.rs (C2Rust translation) + main.rs (copy) ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Processing folders: ${PROCESS_FOLDERS[*]}"
if [ "$FORCE_REGENERATE" = "true" ]; then
    echo "Mode: FORCE (regenerate existing files)"
fi
echo ""

# Global statistics
TOTAL=0
SUCCESS=0
SKIPPED=0
FAILED=0

# Process each specified folder
for FOLDER in "${PROCESS_FOLDERS[@]}"; do
    FOLDER_PATH="$PROJECT_DIR/$FOLDER"
    
    if [ ! -d "$FOLDER_PATH" ]; then
        echo -e "${RED}❌ Error: $FOLDER_PATH not found${NC}"
        continue
    fi
    
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}📁 Processing: $FOLDER/${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    FOLDER_TOTAL=0
    FOLDER_SUCCESS=0
    FOLDER_SKIPPED=0
    FOLDER_FAILED=0
    
    # Process each directory in the folder
    for dir in "$FOLDER_PATH"/*; do
        if [ -d "$dir" ]; then
            DIR_NAME=$(basename "$dir")
            FOLDER_TOTAL=$((FOLDER_TOTAL + 1))
            TOTAL=$((TOTAL + 1))
            
            # Check if a.c exists
            if [ ! -f "$dir/a.c" ]; then
                echo -e "${YELLOW}⊘ SKIP ${DIR_NAME}${NC}: No a.c found"
                FOLDER_SKIPPED=$((FOLDER_SKIPPED + 1))
                SKIPPED=$((SKIPPED + 1))
                continue
            fi
            
            # Check if main.c2rust.rs already exists (unless --force)
            if [ -f "$dir/main.c2rust.rs" ] && [ "$FORCE_REGENERATE" = "false" ]; then
                echo -e "${YELLOW}⊘ SKIP ${DIR_NAME}${NC}: main.c2rust.rs already exists"
                FOLDER_SKIPPED=$((FOLDER_SKIPPED + 1))
                SKIPPED=$((SKIPPED + 1))
                continue
            fi
            
            echo -n "[${FOLDER}] Processing ${DIR_NAME}... "
            
            cd "$dir" || exit 1
            
            # Backup existing files
            rm -f compile_commands.json c2rust_transpile.log
            [ -f main.c2rust.rs ] && cp main.c2rust.rs main.c2rust.rs.bak
            
            # Generate compile_commands.json
            # Using a.c as the source file; c2rust will generate a.rs
            echo "[{\"arguments\":[\"cc\",\"-c\",\"-pthread\",\"-o\",\"main.o\",\"a.c\"],\"directory\":\"$(pwd)\",\"file\":\"a.c\"}]" > compile_commands.json
            
            # Run c2rust transpile
            if ! "$C2RUST_BIN" transpile -e compile_commands.json > c2rust_transpile.log 2>&1; then
                echo -e "${RED}❌ FAILED${NC}"
                echo "   Error: c2rust transpile failed"
                cat c2rust_transpile.log | head -10 | sed 's/^/   /'
                FOLDER_FAILED=$((FOLDER_FAILED + 1))
                FAILED=$((FAILED + 1))
                cd "$PROJECT_DIR" || exit 1
                continue
            fi
            
            # Check if a.rs was generated
            if [ ! -f a.rs ]; then
                echo -e "${RED}❌ FAILED${NC}"
                echo "   Error: No a.rs generated by c2rust"
                FOLDER_FAILED=$((FOLDER_FAILED + 1))
                FAILED=$((FAILED + 1))
                cd "$PROJECT_DIR" || exit 1
                continue
            fi
            
            # Rename a.rs to main.c2rust.rs (the C2Rust translation output)
            mv a.rs main.c2rust.rs
            
            # Verify main.c2rust.rs is not empty
            if [ ! -s main.c2rust.rs ]; then
                echo -e "${RED}❌ FAILED${NC}"
                echo "   Error: Generated main.c2rust.rs is empty"
                FOLDER_FAILED=$((FOLDER_FAILED + 1))
                FAILED=$((FAILED + 1))
                # Restore backup if exists
                [ -f main.c2rust.rs.bak ] && mv main.c2rust.rs.bak main.c2rust.rs
                cd "$PROJECT_DIR" || exit 1
                continue
            fi
            
            # CRITICAL: Copy main.c2rust.rs to main.rs
            # This ensures pub mod main; in c2rust-lib.rs can find the module
            cp main.c2rust.rs main.rs
            
            # Verify main.rs was created
            if [ ! -f main.rs ]; then
                echo -e "${RED}❌ FAILED${NC}"
                echo "   Error: Failed to create main.rs from main.c2rust.rs"
                FOLDER_FAILED=$((FOLDER_FAILED + 1))
                FAILED=$((FAILED + 1))
                cd "$PROJECT_DIR" || exit 1
                continue
            fi
            
            # Ensure c2rust-lib.rs has proper module declaration
            if [ ! -f c2rust-lib.rs ]; then
                echo -e "${RED}❌ FAILED${NC}"
                echo "   Error: c2rust-lib.rs missing"
                FOLDER_FAILED=$((FOLDER_FAILED + 1))
                FAILED=$((FAILED + 1))
                cd "$PROJECT_DIR" || exit 1
                continue
            fi
            
            if ! grep -q "pub mod main" c2rust-lib.rs; then
                echo -e "${RED}❌ FAILED${NC}"
                echo "   Error: c2rust-lib.rs doesn't declare 'pub mod main'"
                FOLDER_FAILED=$((FOLDER_FAILED + 1))
                FAILED=$((FAILED + 1))
                cd "$PROJECT_DIR" || exit 1
                continue
            fi
            
            # Clean up temporary files
            rm -f compile_commands.json c2rust_transpile.log main.c2rust.rs.bak
            
            echo -e "${GREEN}✅ SUCCESS${NC} (main.c2rust.rs + main.rs created)"
            FOLDER_SUCCESS=$((FOLDER_SUCCESS + 1))
            SUCCESS=$((SUCCESS + 1))
            
            cd "$PROJECT_DIR" || exit 1
        fi
    done
    
    # Print folder summary
    echo ""
    echo -e "${BLUE}Summary for $FOLDER/:${NC}"
    echo "  Total:                 $FOLDER_TOTAL"
    echo -e "  ${GREEN}Successfully generated${NC}: $FOLDER_SUCCESS"
    echo -e "  ${YELLOW}Skipped${NC}:                $FOLDER_SKIPPED"
    echo -e "  ${RED}Failed${NC}:                 $FOLDER_FAILED"
    echo ""
done

# Print global summary
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}📊 OVERALL SUMMARY${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo "  Total directories:     $TOTAL"
echo -e "  ${GREEN}Successfully generated${NC}: $SUCCESS"
echo -e "  ${YELLOW}Skipped${NC}:                $SKIPPED"
echo -e "  ${RED}Failed${NC}:                 $FAILED"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All examples processed successfully!${NC}"
    echo ""
    echo "File structure created in each directory:"
    echo "  ├── main.c2rust.rs   (C2Rust transpiled output)"
    echo "  ├── main.rs          (copy of main.c2rust.rs - used by Rust module system)"
    echo "  ├── main.concrat.rs  (existing - no changes)"
    echo "  └── c2rust-lib.rs    (module declarations: pub mod main;)"
    echo ""
    echo "Next steps:"
    echo "  1. Verify with: cargo build (in any example directory)"
    echo "  2. Test the refactoring pipeline: ./LLM/run.sh"
    exit 0
else
    echo -e "${RED}❌ Some examples failed to process${NC}"
    exit 1
fi
