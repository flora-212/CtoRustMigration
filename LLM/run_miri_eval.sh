#!/bin/bash
# Run miri evaluation and save detailed reports
# Usage: ./run_miri_eval.sh [--output-dir DIR] [--timeout SECONDS]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LLM_DIR="$(dirname "$SCRIPT_DIR")"
EVAL_SCRIPT="$SCRIPT_DIR/evaluation/miri_eval.py"

# Defaults
OUTPUT_DIR=""
TIMEOUT=90

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--output-dir DIR] [--timeout SECONDS]"
            exit 1
            ;;
    esac
done

# Determine output directory
if [ -z "$OUTPUT_DIR" ]; then
    if [ -f "$LLM_DIR/.last_refactor_output" ]; then
        OUTPUT_DIR=$(cat "$LLM_DIR/.last_refactor_output")
    else
        echo "❌ No output directory specified and .last_refactor_output not found"
        echo "Usage: $0 --output-dir DIR [--timeout SECONDS]"
        exit 1
    fi
fi

if [ ! -d "$OUTPUT_DIR" ]; then
    echo "❌ Output directory not found: $OUTPUT_DIR"
    exit 1
fi

echo "📁 Testing directory: $OUTPUT_DIR"
echo "⏱️  Timeout per test: ${TIMEOUT}s"
echo ""

# Create result directory
RESULT_DIR="$OUTPUT_DIR/miri_results"
mkdir -p "$RESULT_DIR"

REPORT_MD="$RESULT_DIR/miri_report.md"
REPORT_JSON="$RESULT_DIR/miri_results.json"
REPORT_LOG="$RESULT_DIR/miri_test.log"

echo "Running miri evaluation..."
python3 "$EVAL_SCRIPT" \
    --output-dir "$OUTPUT_DIR" \
    --timeout "$TIMEOUT" \
    --report-output "$REPORT_MD" \
    --json-output "$REPORT_JSON" \
    2>&1 | tee "$REPORT_LOG"

echo ""
echo "📊 Results saved to:"
echo "   📝 Markdown Report: $REPORT_MD"
echo "   📋 JSON Results: $REPORT_JSON"
echo "   📑 Full Log: $REPORT_LOG"
echo ""

# Display summary
if [ -f "$REPORT_JSON" ]; then
    PASSED=$(jq '.passed' "$REPORT_JSON" 2>/dev/null || echo "?")
    FAILED=$(jq '.failed' "$REPORT_JSON" 2>/dev/null || echo "?")
    TOTAL=$(jq '.total' "$REPORT_JSON" 2>/dev/null || echo "?")
    echo "✅ Summary: $PASSED/$TOTAL passed, $FAILED failed"
fi
