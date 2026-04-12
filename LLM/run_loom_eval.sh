#!/bin/bash
# Convenient script to run loom concurrency testing on generated code
# 
# Usage:
#   ./run_loom_eval.sh                           # Use last refactor output
#   ./run_loom_eval.sh /path/to/output/dir       # Test specific output directory
#   ./run_loom_eval.sh --help                    # Show help

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Check for help
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Loom Concurrency Testing for LLM-generated Rust Code"
    echo ""
    echo "Usage:"
    echo "  ./run_loom_eval.sh                         # Use last refactor output"
    echo "  ./run_loom_eval.sh /path/to/output/dir    # Test specific directory"
    echo "  ./run_loom_eval.sh --help                 # Show this help"
    echo ""
    echo "Options:"
    echo "  --from-last                  # Explicitly use last refactor output"
    echo "  --timeout SECONDS            # Override timeout (default: 600s)"
    echo ""
    echo "Examples:"
    echo "  ./run_loom_eval.sh                                      # All examples from last run"
    echo "  ./run_loom_eval.sh /home/guoxy/concrat/LLM/result/...  # Specific directory"
    echo "  ./run_loom_eval.sh --from-last --timeout 300           # Faster timeout"
    echo ""
    echo "⚠️  WARNING: Loom testing is VERY SLOW!"
    echo "   Expected time: 10+ minutes per example"
    echo "   Total time for all examples could be HOURS"
    echo ""
    exit 0
fi

OUTPUT_DIR=""
EXTRA_ARGS=""

# Parse arguments
while [ $# -gt 0 ]; do
    case "$1" in
        --timeout|--report-output|--json-output)
            # These arguments take a value
            EXTRA_ARGS="$EXTRA_ARGS $1 $2"
            shift 2
            ;;
        --from-last)
            EXTRA_ARGS="$EXTRA_ARGS --from-last"
            shift
            ;;
        -*)
            EXTRA_ARGS="$EXTRA_ARGS $1"
            shift
            ;;
        *)
            OUTPUT_DIR="$1"
            shift
            ;;
    esac
done

echo ""
echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                    🧪 Loom Concurrency Testing                             ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Determine output directory
if [ ! -z "$OUTPUT_DIR" ]; then
    if [ ! -d "$OUTPUT_DIR" ]; then
        echo "❌ Error: Directory not found: $OUTPUT_DIR"
        exit 1
    fi
    echo "📁 Testing directory: $OUTPUT_DIR"
elif [ -f "$SCRIPT_DIR/.last_refactor_output" ]; then
    OUTPUT_DIR=$(cat "$SCRIPT_DIR/.last_refactor_output")
    if [ -d "$OUTPUT_DIR" ]; then
        echo "📁 Using last refactor output: $OUTPUT_DIR"
    else
        echo "❌ Error: Last output directory not found: $OUTPUT_DIR"
        exit 1
    fi
else
    echo "❌ Error: No output directory specified and no .last_refactor_output file found"
    exit 1
fi

echo ""
echo "⏱️  Start time: $(date)"
echo "⚠️  This analysis may take a VERY LONG TIME (hours for all examples)"
echo ""

# Run loom evaluation
python3 "$SCRIPT_DIR/evaluation/loom_eval.py" --output-dir "$OUTPUT_DIR" $EXTRA_ARGS

echo ""
echo "⏱️  End time: $(date)"
echo "✅ Loom testing completed!"
echo ""
echo "📁 Results saved to: $OUTPUT_DIR/evaluation/"
echo "   - loom_test_report.md (readable table)"
echo "   - loom_test_results.json (detailed JSON)"
