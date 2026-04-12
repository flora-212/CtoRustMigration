#!/bin/bash
# Usage: ./run_evaluation.sh [prompt_index] [options]
#   prompt_index: 0, 1, ... (default: 0)
#   
# Options:
#   --force              force re-run all evaluations
#   --force-compare      force compare_all.py only
#   --force-clippy       force clippy_concurrency_eval.py only
#   --clear              clear and force both compare and clippy
#   --output-dir DIR     use specific output directory
#   --timestamp TM       use specific timestamp directory
#   --eval-tools TOOLS   specify evaluation tools (default: all)
#                        comma-separated: compare,clippy,safety,miri,loom
#                        or: fast (compare+clippy), full (all), none
#   --miri-timeout SEC   miri timeout in seconds (default: 300)
#   --loom-timeout SEC   loom timeout in seconds (default: 600)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RESULT_DIR="$SCRIPT_DIR/result"
PROMPT_IDX="${1:-0}"
FORCE_COMPARE=""
FORCE_CLIPPY=""
CLEAR_FLAG=""
TIMESTAMP=""
OUTPUT_DIR=""
EVAL_TOOLS="all"           # Default: run all evaluations
MIRI_TIMEOUT="300"         # 5 minutes
LOOM_TIMEOUT="600"         # 10 minutes

# Parse arguments
i=2  # Start from second argument (first is prompt_idx)
while [ $i -le $# ]; do
    arg="${!i}"
    case "$arg" in
        --force)
            FORCE_COMPARE="--force"
            FORCE_CLIPPY="--force"
            ;;
        --force-compare)
            FORCE_COMPARE="--force"
            ;;
        --force-clippy)
            FORCE_CLIPPY="--force"
            ;;
        --clear)
            CLEAR_FLAG="--clear"
            FORCE_COMPARE="--force"
            FORCE_CLIPPY="--force"
            ;;
        --output-dir)
            i=$((i+1))
            if [ $i -le $# ]; then
                OUTPUT_DIR="${!i}"
            fi
            ;;
        --timestamp)
            i=$((i+1))
            if [ $i -le $# ]; then
                TIMESTAMP="${!i}"
            fi
            ;;
        --eval-tools)
            i=$((i+1))
            if [ $i -le $# ]; then
                EVAL_TOOLS="${!i}"
            fi
            ;;
        --miri-timeout)
            i=$((i+1))
            if [ $i -le $# ]; then
                MIRI_TIMEOUT="${!i}"
            fi
            ;;
        --loom-timeout)
            i=$((i+1))
            if [ $i -le $# ]; then
                LOOM_TIMEOUT="${!i}"
            fi
            ;;
    esac
    i=$((i+1))
done

# Use provided output dir, or try to find timestamped directory
if [ ! -z "$OUTPUT_DIR" ]; then
    if [ ! -d "$OUTPUT_DIR" ]; then
        echo "❌ Output directory not found: $OUTPUT_DIR"
        exit 1
    fi
    TIMESTAMPED_DIR="$OUTPUT_DIR"
else
    # Try to read from .last_refactor_output file (preferred method)
    LAST_OUTPUT_FILE="$SCRIPT_DIR/.last_refactor_output"
    if [ -f "$LAST_OUTPUT_FILE" ]; then
        LAST_OUTPUT_DIR=$(cat "$LAST_OUTPUT_FILE")
        if [ ! -z "$LAST_OUTPUT_DIR" ] && [ -d "$LAST_OUTPUT_DIR" ]; then
            TIMESTAMPED_DIR="$LAST_OUTPUT_DIR"
            echo "📁 Found output directory from .last_refactor_output file"
        fi
    fi
    
    # If not found yet, try to find timestamp if provided
    if [ -z "$TIMESTAMPED_DIR" ] && [ ! -z "$TIMESTAMP" ]; then
        TIMESTAMPED_DIR="$RESULT_DIR/$TIMESTAMP"
    fi
    
    # Last resort: find the latest timestamped directory for this prompt
    # This now matches patterns like: {timestamp}_{prompt_idx}, {timestamp}_{prompt_idx}_{strategy}
    if [ -z "$TIMESTAMPED_DIR" ]; then
        # First try to find directories that end with the prompt index
        FOUND_DIR=$(find "$RESULT_DIR" -maxdepth 1 -type d -name "*_${PROMPT_IDX}*" | sort -r | head -1)
        if [ ! -z "$FOUND_DIR" ]; then
            TIMESTAMPED_DIR="$FOUND_DIR"
            TIMESTAMP=$(basename "$FOUND_DIR")
        else
            echo "❌ No timestamped directory found for prompt ${PROMPT_IDX}"
            echo "   Searched for pattern: *_${PROMPT_IDX}* in $RESULT_DIR"
            exit 1
        fi
    fi
fi

if [ ! -d "$TIMESTAMPED_DIR" ]; then
    echo "❌ Timestamped directory not found: $TIMESTAMPED_DIR"
    exit 1
fi

# Ensure evaluation directory exists
mkdir -p "$TIMESTAMPED_DIR/evaluation"

set -e

echo "📁 Using timestamped output directory: $TIMESTAMPED_DIR"
echo "🔧 Evaluation tools: $EVAL_TOOLS"
echo ""

# Expand eval tools aliases
case "$EVAL_TOOLS" in
    all)
        EVAL_TOOLS="compare,clippy,safety,miri,loom"
        ;;
    fast)
        EVAL_TOOLS="compare,clippy"
        ;;
    full)
        EVAL_TOOLS="compare,clippy,safety,miri,loom"
        ;;
    none)
        EVAL_TOOLS=""
        ;;
    *)
        # User specified custom list - use as-is
        ;;
esac

EVAL_DIR="$SCRIPT_DIR/evaluation"

# Function to check if tool should be run
should_run_tool() {
    local tool=$1
    if [[ ",$EVAL_TOOLS," == *",$tool,"* ]]; then
        return 0
    fi
    return 1
}

# Run evaluations based on EVAL_TOOLS
echo "=== Running compare_all.py (baseline comparison) ==="
if should_run_tool "compare"; then
    python3 "$EVAL_DIR/compare_all.py" "$PROMPT_IDX" $FORCE_COMPARE $CLEAR_FLAG --llm-output-dir "$TIMESTAMPED_DIR"
else
    echo "⏭️  Skipped (not in EVAL_TOOLS)"
fi

echo ""
echo "=== Running clippy_concurrency_eval.py (code safety analysis) ==="
if should_run_tool "clippy"; then
    python3 "$EVAL_DIR/clippy_concurrency_eval.py" "$PROMPT_IDX" $FORCE_CLIPPY --llm-output-dir "$TIMESTAMPED_DIR"
else
    echo "⏭️  Skipped (not in EVAL_TOOLS)"
fi

echo ""
echo "=== Running loom_eval.py (loom concurrency testing) ==="
if should_run_tool "loom"; then
    echo "⚠️  Note: Loom testing is VERY SLOW (10+ min per example). Timeout: ${LOOM_TIMEOUT}s"
    python3 "$EVAL_DIR/loom_eval.py" \
        --output-dir "$TIMESTAMPED_DIR" \
        --report-output "$TIMESTAMPED_DIR/evaluation/loom_report.md" \
        --json-output "$TIMESTAMPED_DIR/evaluation/loom_results.json" \
        --timeout "$LOOM_TIMEOUT" \
        2>&1 || {
        echo "⚠️  Loom evaluation completed with warnings or failures (this is normal if some tests fail)"
    }
else
    echo "⏭️  Skipped (not in EVAL_TOOLS)"
fi

echo ""
echo "=== Running miri_eval.py (undefined behavior detection) ==="
if should_run_tool "miri"; then
    python3 "$EVAL_DIR/miri_eval.py" \
        --output-dir "$TIMESTAMPED_DIR" \
        --report-output "$TIMESTAMPED_DIR/evaluation/miri_report.md" \
        --json-output "$TIMESTAMPED_DIR/evaluation/miri_results.json" \
        --timeout "$MIRI_TIMEOUT" \
        2>&1 || {
        echo "⚠️  Miri evaluation completed with warnings or failures (this is normal if some tests fail)"
    }
else
    echo "⏭️  Skipped (not in EVAL_TOOLS)"
fi

echo ""
echo "=== Verification: Checking evaluation results in timestamped directory ==="
if [ -d "$TIMESTAMPED_DIR/evaluation" ]; then
    eval_count=$(find "$TIMESTAMPED_DIR/evaluation" -maxdepth 1 -type f \( -name '*.json' -o -name '*.md' \) | wc -l)
    if [ $eval_count -gt 0 ]; then
        echo "✅ Found $eval_count evaluation result files:"
        find "$TIMESTAMPED_DIR/evaluation" -maxdepth 1 -type f \( -name '*.json' -o -name '*.md' \) | while read file; do
            echo "  - $(basename "$file")"
        done
    else
        echo "⚠️  No evaluation result files found in $TIMESTAMPED_DIR/evaluation/"
    fi
else
    echo "⚠️  Evaluation directory not found: $TIMESTAMPED_DIR/evaluation/"
fi

echo ""
echo "=== Evaluation done ==="
echo "📁 Results saved to: $TIMESTAMPED_DIR/evaluation/"
