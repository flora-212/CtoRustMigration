#!/bin/bash
# Usage: ./run_evaluation.sh [prompt_index] [--force] [--force-compare] [--force-clippy] [--clear]
#   prompt_index: 0, 1, ... (default: 0)
#   --force: force both compare_all.py and clippy_concurrency_eval.py
#   --force-compare: force compare_all.py only
#   --force-clippy: force clippy_concurrency_eval.py only
#   --clear: pass through to compare_all.py and also force both test steps

set -e

PROMPT_IDX="${1:-0}"
FORCE_COMPARE=""
FORCE_CLIPPY=""
CLEAR_FLAG=""

for arg in "$@"; do
    if [ "$arg" = "--force" ]; then
        FORCE_COMPARE="--force"
        FORCE_CLIPPY="--force"
    fi
    if [ "$arg" = "--force-compare" ]; then
        FORCE_COMPARE="--force"
    fi
    if [ "$arg" = "--force-clippy" ]; then
        FORCE_CLIPPY="--force"
    fi
    if [ "$arg" = "--clear" ]; then
        CLEAR_FLAG="--clear"
        FORCE_COMPARE="--force"
        FORCE_CLIPPY="--force"
    fi
done

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"/evaluation

echo "=== Running compare_all.py ==="
python3 "$SCRIPT_DIR/compare_all.py" "$PROMPT_IDX" $FORCE_COMPARE $CLEAR_FLAG

echo ""
echo "=== Running clippy_concurrency_eval.py ==="
python3 "$SCRIPT_DIR/clippy_concurrency_eval.py" "$PROMPT_IDX" $FORCE_CLIPPY

echo ""
echo "=== Evaluation done ==="
