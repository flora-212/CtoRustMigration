#!/bin/bash
# Complete workflow script for iterative generation-validation-evaluation
#
# Usage:
#   ./run.sh                                          # Use default parameters (prompt=1, validate=true)
#   ./run.sh 0                                        # Use SYSTEM_PROMPT_0
#   ./run.sh 1 --validate                             # Explicitly enable validation (enabled by default)
#   ./run.sh 1 --validate --strategy comprehensive     # Comprehensive validation
#   ./run.sh 1 --validate --max-iterations 5           # Custom iteration count
#   ./run.sh 1 --force                                # Force regeneration and evaluation
#   ./run.sh 1 --force --strategy lock_safety         # Force + lock safety strategy
#   ./run.sh 1 --no-validate                          # Disable validation loop

set -e

# ════════════════════════════════════════════════════════════════════════════
# Parameter parsing
# ════════════════════════════════════════════════════════════════════════════

# Default values
PROMPT_IDX=3                          # Default to prompt 3
VALIDATE_FLAG="--validate"            # Enable validation by default
STRATEGY="compile"                    # Validation strategy
MAX_ITERATIONS=5                      # Maximum iteration count
FORCE_REWRITE=""
FORCE_EVAL=""
FORCE_GENERATE=""
CLEAR_FLAG=""
VERBOSE=""

# Check for --help first
for arg in "$@"; do
    if [ "$arg" = "--help" ]; then
        echo "Complete workflow script for iterative generation-validation-evaluation"
        echo ""
        echo "Usage:"
        echo "  ./run.sh                                    # Default: prompt=1, validate=true"
        echo "  ./run.sh 0                                  # Use SYSTEM_PROMPT_0"
        echo "  ./run.sh 1 --validate                       # Enable validation"
        echo "  ./run.sh 1 --no-validate                    # Disable validation"
        echo "  ./run.sh 1 --strategy {compile|safety|...}  # Validation strategy"
        echo "  ./run.sh 1 --max-iterations 5               # Iteration count"
        echo "  ./run.sh 1 --force                          # Force re-execution"
        echo "  ./run.sh 1 --verbose                        # Verbose output"
        echo ""
        echo "Strategies:"
        echo "  compile        - Compilation check (fastest)"
        echo "  safety         - Safety check"
        echo "  lock_safety    - Lock safety analysis"
        echo "  comprehensive  - Comprehensive check"
        exit 0
    fi
done

# Process first positional argument (prompt_idx)
if [ $# -gt 0 ] && [ "$1" != "--"* ]; then
    PROMPT_IDX="$1"
    shift
fi

# Process all flags
while [ $# -gt 0 ]; do
    case "$1" in
        --force)
            FORCE_REWRITE="--force"
            FORCE_EVAL="--force"
            FORCE_GENERATE="--force"
            ;;
        --force-rewrite)
            FORCE_REWRITE="--force"
            ;;
        --force-eval)
            FORCE_EVAL="--force"
            ;;
        --force-generate)
            FORCE_GENERATE="--force"
            ;;
        --clear)
            CLEAR_FLAG="--clear"
            FORCE_REWRITE="--force"
            FORCE_EVAL="--force"
            FORCE_GENERATE="--force"
            ;;
        --validate)
            VALIDATE_FLAG="--validate"
            ;;
        --no-validate)
            VALIDATE_FLAG=""
            ;;
        --strategy)
            if [ -z "$2" ]; then
                echo "❌ Error: --strategy requires an argument"
                exit 1
            fi
            STRATEGY="$2"
            shift
            ;;
        --max-iterations)
            if [ -z "$2" ]; then
                echo "❌ Error: --max-iterations requires an argument"
                exit 1
            fi
            MAX_ITERATIONS="$2"
            shift
            ;;
        --verbose)
            VERBOSE="--verbose"
            ;;
        *)
            echo "❌ Unknown argument: $1"
            exit 1
            ;;
    esac
    shift
done

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# ════════════════════════════════════════════════════════════════════════════
# Welcome banner
# ════════════════════════════════════════════════════════════════════════════

echo ""
echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║           Complete Workflow: Generation - Validation - Evaluation          ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "✅ Configuration:"
echo "   - Prompt Index: $PROMPT_IDX"
echo "   - Validation: ${VALIDATE_FLAG:-(disabled)}"
if [ ! -z "$VALIDATE_FLAG" ]; then
    echo "   - Strategy: $STRATEGY"
    echo "   - Max Iterations: $MAX_ITERATIONS"
fi
echo "   - Force Rewrite: ${FORCE_REWRITE:-(no)}"
echo "   - Force Eval: ${FORCE_EVAL:-(no)}"
echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 1: Start Ollama
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📡 Step 1: Start Ollama LLM Service"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if pgrep -x ollama > /dev/null 2>&1; then
    echo "✅ Ollama is already running"
else
    echo "🚀 Starting Ollama..."
    ollama serve > /dev/null 2>&1 &
    OLLAMA_PID=$!
    echo "   Ollama PID: $OLLAMA_PID"
    echo "   Waiting for Ollama to be ready..."
    for i in $(seq 1 30); do
        if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
            echo "✅ Ollama is ready"
            break
        fi
        sleep 1
        if [ $i -eq 30 ]; then
            echo "❌ Ollama startup timeout"
            exit 1
        fi
    done
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 2: Code generation + iterative validation
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔄 Step 2: Generate Code (SYSTEM_PROMPT_${PROMPT_IDX})"
if [ ! -z "$VALIDATE_FLAG" ]; then
    echo "   + Iterative validation loop (strategy: $STRATEGY, max iterations: $MAX_ITERATIONS)"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ ! -z "$VALIDATE_FLAG" ]; then
    # Use validation loop
    python3 "$SCRIPT_DIR/refractor.py" "$PROMPT_IDX" \
        $VALIDATE_FLAG \
        --strategy "$STRATEGY" \
        --max-iterations "$MAX_ITERATIONS" \
        $FORCE_REWRITE \
        $VERBOSE
else
    # No validation loop
    python3 "$SCRIPT_DIR/refractor.py" "$PROMPT_IDX" $FORCE_REWRITE $VERBOSE
fi

REFACTOR_STATUS=$?
if [ $REFACTOR_STATUS -eq 0 ]; then
    echo "✅ Code generation completed"
else
    echo "❌ Code generation failed (exit code: $REFACTOR_STATUS)"
    exit $REFACTOR_STATUS
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 3: Run evaluation suite
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Step 3: Run Evaluation Suite"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Read the output directory from refractor.py
REFACTOR_OUTPUT_DIR=""
LAST_OUTPUT_FILE="$SCRIPT_DIR/.last_refactor_output"
if [ -f "$LAST_OUTPUT_FILE" ]; then
    REFACTOR_OUTPUT_DIR=$(cat "$LAST_OUTPUT_FILE")
    echo "📁 Found refactor output directory: $REFACTOR_OUTPUT_DIR"
fi

bash "$SCRIPT_DIR/run_evaluation.sh" "$PROMPT_IDX" $FORCE_EVAL $CLEAR_FLAG --output-dir "$REFACTOR_OUTPUT_DIR"

EVAL_STATUS=$?
if [ $EVAL_STATUS -eq 0 ]; then
    echo "✅ Evaluation suite completed"
else
    echo "⚠️  Evaluation suite report (exit code: $EVAL_STATUS)"
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 4: Generate comparison report
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📈 Step 4: Generate Comparison Report"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Read the output directory from refractor.py for comparison report generation
if [ -z "$REFACTOR_OUTPUT_DIR" ]; then
    LAST_OUTPUT_FILE="$SCRIPT_DIR/.last_refactor_output"
    if [ -f "$LAST_OUTPUT_FILE" ]; then
        REFACTOR_OUTPUT_DIR=$(cat "$LAST_OUTPUT_FILE")
    fi
fi

if [ ! -z "$REFACTOR_OUTPUT_DIR" ]; then
    python3 "$SCRIPT_DIR/evaluation/generate_comparison.py" "$PROMPT_IDX" $FORCE_GENERATE --llm-output-dir "$REFACTOR_OUTPUT_DIR"
else
    python3 "$SCRIPT_DIR/evaluation/generate_comparison.py" "$PROMPT_IDX" $FORCE_GENERATE
fi

COMPARE_STATUS=$?
if [ $COMPARE_STATUS -eq 0 ]; then
    echo "✅ Comparison report generated"
else
    echo "❌ Comparison report generation failed (exit code: $COMPARE_STATUS)"
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Final summary
# ════════════════════════════════════════════════════════════════════════════

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                     ✅ Workflow Execution Complete!                        ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "📁 Output file locations:"
echo ""
echo "   📝 Generated code and evaluation results (recommended):"
LAST_OUTPUT=$([ -f "$SCRIPT_DIR/.last_refactor_output" ] && cat "$SCRIPT_DIR/.last_refactor_output" || echo "result/{timestamp}_{PROMPT_IDX}")
echo "      $LAST_OUTPUT/"
echo "      ├── config.json                          # Parameters and configuration"
echo "      ├── examples/                            # Generation results for all examples"
echo "      │   ├── {example_name}/"
echo "      │   │   ├── round1.rs, round2.rs, ...     # Iteration process"
echo "      │   │   └── final.rs                      # Final version ✅"
echo "      │   └── ..."
echo "      └── evaluation/                          # Evaluation results (auto-generated)"
echo "          ├── comparison_report.json"
echo "          ├── comparison_report.md"
echo "          ├── clippy_concurrency_report.json"
echo "          └── clippy_concurrency_report.md"
echo ""
echo "   💡 Notes:"
echo "      - All generated code is read from final.rs (no longer depends on examples dir copies)"
echo "      - Each run generates a new timestamped directory for multi-version management"
echo "      - Evaluation script automatically reads results from generated output directory"
echo ""
echo "🔍 View results (new format):"
LATEST_DIR=$(find result -maxdepth 1 -type d -name "*_${PROMPT_IDX}" | sort -r | head -1)
if [ ! -z "$LATEST_DIR" ]; then
    echo "   # Latest output directory: $LATEST_DIR"
    echo "   cat $LATEST_DIR/config.json | jq ."
    echo ""
    echo "   # View comparison report"
    echo "   cat $LATEST_DIR/evaluation/comparison_report.json | jq ."
    echo ""
    echo "   # View all evaluation results"
    echo "   ls $LATEST_DIR/evaluation/"
fi
echo ""
echo "🧹 Cleanup generated files:"
echo "   # View all generated files"
echo "   python3 cleanup.py --all --dry-run"
echo ""
echo "   # Delete all versions"
echo "   python3 cleanup.py --all --yes"
echo ""
