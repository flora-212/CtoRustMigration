#!/bin/bash
# Run all a.c files in examples and examples_negative directories
# Save output to output.txt in each directory
# Do not modify .c files even if compilation fails
# Skip examples that take too long (timeout)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/examples"
EXAMPLES_NEG_DIR="$SCRIPT_DIR/examples_negative"

# Timeout settings
COMPILE_TIMEOUT=30    # seconds for compilation
RUN_TIMEOUT=10        # seconds for execution

# Parse arguments
PATTERN=""
HELP=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help)
            HELP=true
            shift
            ;;
        -p|--pattern)
            PATTERN="$2"
            shift 2
            ;;
        *)
            PATTERN="$1"
            shift
            ;;
    esac
done

# Show help if requested
if [ "$HELP" = true ]; then
    cat << EOF
Usage: $0 [OPTIONS] [PATTERN]

Options:
  -h, --help              Show this help message
  -p, --pattern PATTERN   Run examples matching pattern (regex)

Examples:
  $0                      # Run all examples
  $0 array                # Run examples matching 'array'
  $0 "global_.*simple"    # Run examples matching regex
  $0 -p "struct_.*lock"   # Run struct examples with 'lock' in name
  $0 struct_simple        # Run specific example
  
Patterns are treated as regex patterns for matching example names.
EOF
    exit 0
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Summary
TOTAL=0
SUCCESS=0
FAILED=0
TIMEOUT_FAIL=0
FAILED_LIST=()
TIMEOUT_LIST=()
OUTPUT_FILES=()

# Function to check if example name matches pattern
matches_pattern() {
    local name=$1
    if [ -z "$PATTERN" ]; then
        return 0  # Empty pattern matches all
    fi
    if [[ $name =~ $PATTERN ]]; then
        return 0
    fi
    return 1
}

# Function to run one example
run_example() {
    local example_dir=$1
    local example_name=$(basename "$example_dir")
    local a_c="$example_dir/a.c"
    local output_file="$example_dir/output.txt"
    
    # Skip if doesn't match pattern
    if ! matches_pattern "$example_name"; then
        return
    fi
    
    TOTAL=$((TOTAL + 1))
    
    # Check if a.c exists
    if [ ! -f "$a_c" ]; then
        echo -e "${YELLOW}⊘${NC} ${example_name}: a.c not found"
        FAILED=$((FAILED + 1))
        FAILED_LIST+=("$example_name (a.c not found)")
        return
    fi
    
    # Compile with timeout
    local exe_file=$(mktemp)
    local compile_err_file="/tmp/${example_name}_compile.err"
    
    if ! timeout $COMPILE_TIMEOUT gcc -Wno-error=implicit-function-declaration -lpthread -o "$exe_file" "$a_c" 2>"$compile_err_file"; then
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            # Timeout
            echo -e "${YELLOW}⏱${NC} ${example_name}: Compilation timeout (>${COMPILE_TIMEOUT}s)"
            FAILED=$((FAILED + 1))
            TIMEOUT_FAIL=$((TIMEOUT_FAIL + 1))
            TIMEOUT_LIST+=("$example_name (compile timeout)")
            echo "[Compilation timeout after ${COMPILE_TIMEOUT}s]" > "$output_file"
        else
            # Compilation error
            echo -e "${RED}✗${NC} ${example_name}: Compilation failed"
            FAILED=$((FAILED + 1))
            FAILED_LIST+=("$example_name (compile error)")
            cat "$compile_err_file" > "$output_file"
        fi
        rm -f "$exe_file" "$compile_err_file"
        return
    fi
    
    # Run with timeout
    if ! timeout $RUN_TIMEOUT "$exe_file" > "$output_file" 2>&1; then
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            # Timeout during execution
            echo -e "${YELLOW}⏱${NC} ${example_name}: Execution timeout (>${RUN_TIMEOUT}s)"
            FAILED=$((FAILED + 1))
            TIMEOUT_FAIL=$((TIMEOUT_FAIL + 1))
            TIMEOUT_LIST+=("$example_name (execution timeout)")
            echo "[Execution timeout after ${RUN_TIMEOUT}s]" >> "$output_file"
        else
            # Runtime error
            echo -e "${YELLOW}⚠${NC} ${example_name}: Execution returned non-zero"
            FAILED=$((FAILED + 1))
            FAILED_LIST+=("$example_name (runtime error)")
        fi
        rm -f "$exe_file"
        return
    fi
    
    echo -e "${GREEN}✓${NC} ${example_name}"
    SUCCESS=$((SUCCESS + 1))
    OUTPUT_FILES+=("$output_file")
    
    rm -f "$exe_file"
}

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║              Running all a.c files in examples directories                ║"
if [ -n "$PATTERN" ]; then
    echo "║  Pattern: $PATTERN"
fi
echo "║                                                                            ║"
echo "║  Compile timeout: ${COMPILE_TIMEOUT}s | Execution timeout: ${RUN_TIMEOUT}s                     ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Run examples
echo "📁 Processing examples/"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
for example_dir in "$EXAMPLES_DIR"/*/; do
    [ -d "$example_dir" ] || continue
    run_example "$example_dir"
done

echo ""

# Run examples_negative
echo "📁 Processing examples_negative/"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
for example_dir in "$EXAMPLES_NEG_DIR"/*/; do
    [ -d "$example_dir" ] || continue
    run_example "$example_dir"
done

echo ""
echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                           Summary Results                                  ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Print summary
echo "📊 Statistics:"
echo "   • Total: $TOTAL"
echo -e "   • ${GREEN}Passed: $SUCCESS${NC}"
echo -e "   • ${RED}Failed: $FAILED${NC}"
if [ $TIMEOUT_FAIL -gt 0 ]; then
    echo -e "     └─ ${YELLOW}Timeout: $TIMEOUT_FAIL${NC}"
fi
echo ""

# Print failures if any (non-timeout)
COMPILE_ERRORS=0
RUNTIME_ERRORS=0
NOT_FOUND=0

for fail in "${FAILED_LIST[@]}"; do
    if [[ $fail == *"compile error"* ]]; then
        COMPILE_ERRORS=$((COMPILE_ERRORS + 1))
    elif [[ $fail == *"runtime error"* ]]; then
        RUNTIME_ERRORS=$((RUNTIME_ERRORS + 1))
    elif [[ $fail == *"not found"* ]]; then
        NOT_FOUND=$((NOT_FOUND + 1))
    fi
done

if [ $FAILED -gt $TIMEOUT_FAIL ]; then
    echo "❌ Failed examples (non-timeout):"
    if [ $COMPILE_ERRORS -gt 0 ]; then
        echo "   Compilation errors ($COMPILE_ERRORS):"
        for fail in "${FAILED_LIST[@]}"; do
            if [[ $fail == *"compile error"* ]]; then
                echo "   • $fail"
            fi
        done
    fi
    if [ $RUNTIME_ERRORS -gt 0 ]; then
        echo "   Runtime errors ($RUNTIME_ERRORS):"
        for fail in "${FAILED_LIST[@]}"; do
            if [[ $fail == *"runtime error"* ]]; then
                echo "   • $fail"
            fi
        done
    fi
    if [ $NOT_FOUND -gt 0 ]; then
        echo "   Not found ($NOT_FOUND):"
        for fail in "${FAILED_LIST[@]}"; do
            if [[ $fail == *"not found"* ]]; then
                echo "   • $fail"
            fi
        done
    fi
    echo ""
fi

# Print timeout failures if any
if [ $TIMEOUT_FAIL -gt 0 ]; then
    echo "⏱️  Timeout examples (skipped):"
    echo "   (Compile timeout: >${COMPILE_TIMEOUT}s, Execution timeout: >${RUN_TIMEOUT}s)"
    for timeout in "${TIMEOUT_LIST[@]}"; do
        echo "   • $timeout"
    done
    echo ""
fi

# Print output files list
TOTAL_OUTPUT=$(find "$EXAMPLES_DIR" "$EXAMPLES_NEG_DIR" -name "output.txt" -type f 2>/dev/null | wc -l)
echo "📝 Output files created ($TOTAL_OUTPUT):"
find "$EXAMPLES_DIR" "$EXAMPLES_NEG_DIR" -name "output.txt" -type f | sort | while read file; do
    dir=$(dirname "$file")
    name=$(basename "$dir")
    size=$(wc -l < "$file" 2>/dev/null || echo "0")
    echo "   ✓ $name/output.txt ($size lines)"
done

echo ""
echo "✅ Done!"
