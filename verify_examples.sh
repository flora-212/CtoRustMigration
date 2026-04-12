#!/bin/bash
# Verify consistency of a.c execution results
# Runs all a.c files and compares output with existing output.txt
# Does NOT overwrite inconsistent outputs, only reports them at the end

# Parse command line arguments
for arg in "$@"; do
    case "$arg" in
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --help         Show this help message"
            echo ""
            echo "This script verifies all examples by running them and comparing"
            echo "against baseline output.txt files. Timeouts and failures are"
            echo "reported at the end."
            exit 0
            ;;
    esac
done

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/examples"
EXAMPLES_NEG_DIR="$SCRIPT_DIR/examples_negative"

# Timeout settings (same as run_all_examples.sh)
COMPILE_TIMEOUT=30
RUN_TIMEOUT=10

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Summary
TOTAL=0
CONSISTENT=0
INCONSISTENT=0
NO_BASELINE=0
TIMEOUT_CASES=0
BASELINE_MISSING=()
INCONSISTENT_LIST=()
TIMEOUT_LIST=()

# Function to verify one example
verify_example() {
    local example_dir=$1
    local example_name=$(basename "$example_dir")
    local a_c="$example_dir/a.c"
    local baseline_file="$example_dir/output.txt"
    
    TOTAL=$((TOTAL + 1))
    
    # Check if a.c exists
    if [ ! -f "$a_c" ]; then
        echo -e "${YELLOW}⊘${NC} ${example_name}: a.c not found"
        INCONSISTENT=$((INCONSISTENT + 1))
        INCONSISTENT_LIST+=("$example_name (a.c missing)")
        return
    fi
    
    # Check if baseline output exists
    if [ ! -f "$baseline_file" ]; then
        echo -e "${CYAN}?${NC} ${example_name}: No baseline (output.txt missing)"
        NO_BASELINE=$((NO_BASELINE + 1))
        BASELINE_MISSING+=("$example_name")
        return
    fi
    
    # Compile with timeout
    local exe_file=$(mktemp)
    local compile_err_file="/tmp/${example_name}_compile.err"
    
    if ! timeout $COMPILE_TIMEOUT gcc -Wno-error=implicit-function-declaration -lpthread -o "$exe_file" "$a_c" 2>"$compile_err_file"; then
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            # Timeout during compilation
            echo -e "${YELLOW}⏱${NC} ${example_name}: Compile timeout"
            TIMEOUT_CASES=$((TIMEOUT_CASES + 1))
            TIMEOUT_LIST+=("$example_name (compile timeout)")
        else
            # Compilation error
            local baseline_content=$(cat "$baseline_file")
            if [[ "$baseline_content" == *"error:"* ]]; then
                # Baseline was also compilation error
                echo -e "${GREEN}✓${NC} ${example_name}: Consistent (compile error)"
                CONSISTENT=$((CONSISTENT + 1))
            else
                echo -e "${RED}✗${NC} ${example_name}: Inconsistent (was OK, now compile error)"
                INCONSISTENT=$((INCONSISTENT + 1))
                INCONSISTENT_LIST+=("$example_name (compile error on re-run, baseline: OK)")
            fi
        fi
        rm -f "$exe_file" "$compile_err_file"
        return
    fi
    
    # Run with timeout
    local new_output_file=$(mktemp)
    if ! timeout $RUN_TIMEOUT "$exe_file" > "$new_output_file" 2>&1; then
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            # Timeout during execution
            echo -e "${YELLOW}⏱${NC} ${example_name}: Execution timeout"
            TIMEOUT_CASES=$((TIMEOUT_CASES + 1))
            TIMEOUT_LIST+=("$example_name (execution timeout)")
        else
            # Runtime error (non-zero exit)
            local baseline_content=$(cat "$baseline_file")
            if [ -z "$baseline_content" ] || [[ "$baseline_content" == *"Execution timeout"* ]]; then
                echo -e "${RED}✗${NC} ${example_name}: Inconsistent (was empty/timeout, now error)"
                INCONSISTENT=$((INCONSISTENT + 1))
                INCONSISTENT_LIST+=("$example_name (runtime error on re-run)")
            else
                echo -e "${GREEN}✓${NC} ${example_name}: Consistent (runtime error)"
                CONSISTENT=$((CONSISTENT + 1))
            fi
        fi
        rm -f "$exe_file" "$new_output_file"
        return
    fi
    
    # Success: compare outputs
    local baseline_content=$(cat "$baseline_file")
    local new_content=$(cat "$new_output_file")
    
    if [ "$baseline_content" = "$new_content" ]; then
        echo -e "${GREEN}✓${NC} ${example_name}: Consistent output"
        CONSISTENT=$((CONSISTENT + 1))
    else
        echo -e "${RED}✗${NC} ${example_name}: INCONSISTENT OUTPUT"
        INCONSISTENT=$((INCONSISTENT + 1))
        INCONSISTENT_LIST+=("$example_name")
    fi
    
    rm -f "$exe_file" "$new_output_file"
}

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║              Verifying all a.c files against baseline output.txt           ║"
echo "║                                                                            ║"
echo "║  Compile timeout: ${COMPILE_TIMEOUT}s | Execution timeout: ${RUN_TIMEOUT}s                     ║"
echo "║  Timeouts and failures will be reported at the end                         ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Run examples
echo "📁 Verifying examples/"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
for example_dir in "$EXAMPLES_DIR"/*/; do
    [ -d "$example_dir" ] || continue
    verify_example "$example_dir"
done

echo ""

# Run examples_negative  
echo "📁 Verifying examples_negative/"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
for example_dir in "$EXAMPLES_NEG_DIR"/*/; do
    [ -d "$example_dir" ] || continue
    verify_example "$example_dir"
done

echo ""
echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                           Verification Results                             ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Print summary
echo "📊 Statistics:"
echo "   • Total analyzed: $TOTAL"
echo -e "   • ${GREEN}Consistent: $CONSISTENT${NC}"
echo -e "   • ${RED}Inconsistent: $INCONSISTENT${NC}"
if [ $TIMEOUT_CASES -gt 0 ]; then
    echo -e "   • ${YELLOW}Timeout/Failure: $TIMEOUT_CASES${NC}"
fi
if [ $NO_BASELINE -gt 0 ]; then
    echo -e "   • ${CYAN}No Baseline: $NO_BASELINE${NC}"
fi
echo ""

# Calculate consistency percentage (excluding timeout and no-baseline)
TESTABLE=$((TOTAL - NO_BASELINE - TIMEOUT_CASES))
if [ $TESTABLE -gt 0 ]; then
    CONSISTENCY_PERCENT=$((CONSISTENT * 100 / TESTABLE))
    echo "   📈 Consistency Rate (excluding timeouts): $CONSISTENT/$TESTABLE ($CONSISTENCY_PERCENT%)"
    echo ""
fi

# Print timeout cases
if [ $TIMEOUT_CASES -gt 0 ]; then
    echo "⏱️  Timeout/Failure cases:"
    for timeout_case in "${TIMEOUT_LIST[@]}"; do
        echo "   • $timeout_case"
    done
    echo ""
fi

# Print no baseline cases
if [ $NO_BASELINE -gt 0 ]; then
    echo "❓ Examples without baseline (no output.txt):"
    for baseline in "${BASELINE_MISSING[@]}"; do
        echo "   • $baseline"
    done
    echo "      → Run ./run_all_examples.sh to generate baselines"
    echo ""
fi

# Print inconsistencies
if [ $INCONSISTENT -gt 0 ]; then
    echo "🚨 INCONSISTENT CASES (${RED}$INCONSISTENT${NC} found):"
    echo ""
    
    # Detailed information
    local count=1
    for case in "${INCONSISTENT_LIST[@]}"; do
        echo "   [$count] $case"
        
        # Try to get more details
        if [[ "$case" == *"error"* ]] || [[ "$case" == *"timeout"* ]]; then
            # Error case - show type
            echo "       Type: $(echo "$case" | sed 's/.*(\(.*\))/\1/')"
        fi
        
        count=$((count + 1))
    done
    echo ""
    echo "💡 Next steps:"
    echo "   1. Review the inconsistent examples manually"
    echo "   2. For determinism issues: investigate why output varies"
    echo "   3. For new baselines: run ./run_all_examples.sh again"
    echo ""
fi

# Print consistency summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $INCONSISTENT -eq 0 ] && [ $NO_BASELINE -eq 0 ]; then
    echo -e "✅ ${GREEN}All outputs are consistent!${NC}"
else
    echo -e "⚠️  Found ${RED}$INCONSISTENT inconsistencies${NC} and ${CYAN}$NO_BASELINE missing baselines${NC}"
fi
echo ""
echo "✅ Verification complete!"
