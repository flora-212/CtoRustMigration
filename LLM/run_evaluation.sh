#!/bin/bash
# Usage: ./run_evaluation.sh [prompt_index] [--force] [--force-compare] [--force-clippy] [--clear] [--output-dir DIR] [--timestamp TIMESTAMP]
#   prompt_index: 0, 1, ... (default: 0)
#   --force: force both compare_all.py and clippy_concurrency_eval.py
#   --force-compare: force compare_all.py only
#   --force-clippy: force clippy_concurrency_eval.py only
#   --clear: pass through to compare_all.py and also force both test steps
#   --output-dir: use specific output directory (e.g., from refractor.py)
#   --timestamp: use specific timestamp directory (default: auto-detect latest)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RESULT_DIR="$SCRIPT_DIR/result"
PROMPT_IDX="${1:-0}"
FORCE_COMPARE=""
FORCE_CLIPPY=""
CLEAR_FLAG=""
TIMESTAMP=""
OUTPUT_DIR=""

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
    esac
    i=$((i+1))
done

# Use provided output dir or find timestamped directory
if [ ! -z "$OUTPUT_DIR" ]; then
    if [ ! -d "$OUTPUT_DIR" ]; then
        echo "❌ Output directory not found: $OUTPUT_DIR"
        exit 1
    fi
    TIMESTAMPED_DIR="$OUTPUT_DIR"
else
    # Find timestamp if not provided
    if [ -z "$TIMESTAMP" ]; then
        # Find the latest timestamped directory for this prompt
        TIMESTAMP=$(find "$RESULT_DIR" -maxdepth 1 -type d -name "*_${PROMPT_IDX}" | sort -r | head -1 | xargs basename)
        if [ -z "$TIMESTAMP" ]; then
            echo "❌ No timestamped directory found for prompt ${PROMPT_IDX}"
            exit 1
        fi
    fi
    
    TIMESTAMPED_DIR="$RESULT_DIR/$TIMESTAMP"
fi

if [ ! -d "$TIMESTAMPED_DIR" ]; then
    echo "❌ Timestamped directory not found: $TIMESTAMPED_DIR"
    exit 1
fi

echo "📁 Using timestamped output directory: $TIMESTAMPED_DIR"
echo ""

EVAL_DIR="$SCRIPT_DIR/evaluation"

echo "=== Running compare_all.py ==="
python3 "$EVAL_DIR/compare_all.py" "$PROMPT_IDX" $FORCE_COMPARE $CLEAR_FLAG --llm-output-dir "$TIMESTAMPED_DIR"

echo ""
echo "=== Running clippy_concurrency_eval.py ==="
python3 "$EVAL_DIR/clippy_concurrency_eval.py" "$PROMPT_IDX" $FORCE_CLIPPY --llm-output-dir "$TIMESTAMPED_DIR"

echo ""
echo "=== Copying evaluation results to timestamped directory ==="
for file in "$RESULT_DIR/$PROMPT_IDX"/*.json "$RESULT_DIR/$PROMPT_IDX"/*.md; do
    if [ -f "$file" ]; then
        cp "$file" "$TIMESTAMPED_DIR/evaluation/" 2>/dev/null || true
        echo "  - $(basename "$file")"
    fi
done

echo ""
echo "=== Evaluation done ==="
echo "📁 Results saved to: $TIMESTAMPED_DIR/evaluation/"
