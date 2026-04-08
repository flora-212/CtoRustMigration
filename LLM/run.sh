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
STRATEGY="compile"                    # Validation strategy (for backward compatibility)
TOOLS="compile"                       # Validation tools (can be multiple, space-separated)
MAX_ITERATIONS=15                      # Maximum iteration count
MODEL="qwen2.5-coder:14b"                         # Default model
FORCE_REWRITE=""
FORCE_EVAL=""
FORCE_GENERATE=""
CLEAR_FLAG=""
VERBOSE=""
INCLUDE_NEGATIVE=""                 # Include negative samples
NEGATIVE_ONLY=""                    # Only run negative samples

# Check for --help first
for arg in "$@"; do
    if [ "$arg" = "--help" ]; then
        echo "Complete workflow script for iterative generation-validation-evaluation"
        echo ""
        echo "Usage:"
        echo "  ./run.sh                                        # Default: prompt=3, validate=true"
        echo "  ./run.sh 0                                      # Use SYSTEM_PROMPT_0"
        echo "  ./run.sh 1 --validate                           # Enable validation"
        echo "  ./run.sh 1 --no-validate                        # Disable validation"
        echo "  ./run.sh 1 --strategy compile                   # Single tool (backward compatibility)"
        echo "  ./run.sh 1 --tools 'compile clippy'             # Multiple tools (new)"
        echo "  ./run.sh 1 --tools 'compile clippy miri'        # Full validation"
        echo "  ./run.sh 1 --max-iterations 5                   # Iteration count"
        echo "  ./run.sh 1 --model {qwen|...}                   # Specify LLM model (default: qwen)"
        echo "  ./run.sh 1 --force                              # Force re-execution"
        echo "  ./run.sh 1 --include-negative                   # Include negative samples"
        echo "  ./run.sh 1 --negative-only                      # Only run negative samples"
        echo "  ./run.sh 1 --verbose                            # Verbose output"
        echo ""
        echo "Available Tools (can be combined in --tools):"
        echo "  compile        - cargo build - Compilation check (🚀 fast)"
        echo "  clippy         - cargo clippy - Code style/performance (🚀 fast)"
        echo "  miri           - cargo miri - Runtime UB detection (🐌 slow)"
        echo "  loom           - Concurrency testing - Race condition detection (🐢 very slow)"
        echo "  safety         - Static analysis - Unsafe patterns (⚡ very fast)"
        echo "  lock_safety    - Static analysis - Lock usage patterns (⚡ very fast)"
        echo "  comprehensive  - All tools (compile, clippy, safety, lock_safety)"
        echo ""
        echo "Examples:"
        echo "  ./run.sh 3 --tools 'compile'                    # Only compile"
        echo "  ./run.sh 3 --tools 'compile clippy'            # Compile + clippy"
        echo "  ./run.sh 3 --tools 'compile clippy miri'       # Compile + clippy + miri"
        echo "  ./run.sh 3 --tools 'compile clippy loom'       # Compile + clippy + loom"
        echo "  ./run.sh 3 --tools 'safety lock_safety loom'   # Analysis + concurrency testing"
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
            TOOLS="$2"  # Keep consistent with new tools parameter
            shift
            ;;
        --tools)
            if [ -z "$2" ]; then
                echo "❌ Error: --tools requires an argument"
                exit 1
            fi
            TOOLS="$2"
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
        --model)
            if [ -z "$2" ]; then
                echo "❌ Error: --model requires an argument"
                exit 1
            fi
            MODEL="$2"
            shift
            ;;
        --verbose)
            VERBOSE="--verbose"
            ;;
        --include-negative)
            INCLUDE_NEGATIVE="--include-negative"
            ;;
        --negative-only)
            NEGATIVE_ONLY="--negative-only"
            INCLUDE_NEGATIVE="--include-negative"
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

# Print time information for debugging
START_TIME=$(date '+%Y-%m-%d %H:%M:%S')
START_SECONDS=$(date +%s)
echo "⏱️  Start time: $START_TIME"
echo ""

echo "✅ Configuration:"
echo "   - Prompt Index: $PROMPT_IDX"
echo "   - Model: $MODEL"
echo "   - Validation: ${VALIDATE_FLAG:-(disabled)}"
if [ ! -z "$VALIDATE_FLAG" ]; then
    echo "   - Tools: $TOOLS"
    echo "   - Max Iterations: $MAX_ITERATIONS"
    
    # Warn about slow tools (miri, loom)
    SLOW_TOOL_WARNING=""
    if echo "$TOOLS" | grep -q "miri"; then
        SLOW_TOOL_WARNING="yes"
        echo ""
        echo "⚠️  WARNING: 'miri' tool detected - this will be VERY SLOW!"
        echo "    Miri runs code in an interpreter to detect undefined behavior."
        echo "    Expected time: 5-15 minutes per example (may take hours total)"
    fi
    if echo "$TOOLS" | grep -q "loom"; then
        SLOW_TOOL_WARNING="yes"
        echo ""
        echo "⚠️  WARNING: 'loom' tool detected - this will be EXTREMELY SLOW!"
        echo "    Loom tests concurrency via permutation testing (C11 memory model)."
        echo "    Expected time: 10-30 minutes per example (may take MANY HOURS total)"
        echo "    Requires tests written with loom::model(|| {...}) wrapper"
    fi
    if [ ! -z "$SLOW_TOOL_WARNING" ]; then
        echo "    Consider using: --tools 'compile' or --tools 'compile clippy' for faster runs"
        echo ""
    fi
fi
echo "   - Include Negative: ${INCLUDE_NEGATIVE:-(no)}"
if [ ! -z "$NEGATIVE_ONLY" ]; then
    echo "   - Negative Only: yes"
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
STEP1_START=$(date +%s)

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

STEP1_END=$(date +%s)
STEP1_DURATION=$((STEP1_END - STEP1_START))
echo "⏱️  Step 1 took ${STEP1_DURATION}s"
echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 2: Code generation + iterative validation
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔄 Step 2: Generate Code (SYSTEM_PROMPT_${PROMPT_IDX})"
if [ ! -z "$VALIDATE_FLAG" ]; then
    echo "   + Iterative validation loop (tools: $TOOLS, max iterations: $MAX_ITERATIONS)"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
STEP2_START=$(date +%s)

if [ ! -z "$VALIDATE_FLAG" ]; then
    # Use validation loop
    echo "📊 Starting code generation with validation..."
    echo "   (You will see progress updates as each example is processed)"
    python3 "$SCRIPT_DIR/refractor.py" "$PROMPT_IDX" \
        $VALIDATE_FLAG \
        --tools "$TOOLS" \
        --max-iterations "$MAX_ITERATIONS" \
        --model "$MODEL" \
        $INCLUDE_NEGATIVE \
        $NEGATIVE_ONLY \
        $FORCE_REWRITE \
        $VERBOSE
else
    # No validation loop
    python3 "$SCRIPT_DIR/refractor.py" "$PROMPT_IDX" \
        --model "$MODEL" \
        $INCLUDE_NEGATIVE \
        $NEGATIVE_ONLY \
        $FORCE_REWRITE $VERBOSE
fi

STEP2_END=$(date +%s)
STEP2_DURATION=$((STEP2_END - STEP2_START))
echo ""
echo "⏱️  Step 2 (Code Generation) took ${STEP2_DURATION}s ($(( STEP2_DURATION / 60 ))m $(( STEP2_DURATION % 60 ))s)"

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
STEP3_START=$(date +%s)

# Read the output directory from refractor.py
REFACTOR_OUTPUT_DIR=""
LAST_OUTPUT_FILE="$SCRIPT_DIR/.last_refactor_output"
if [ -f "$LAST_OUTPUT_FILE" ]; then
    REFACTOR_OUTPUT_DIR=$(cat "$LAST_OUTPUT_FILE")
    echo "📁 Found refactor output directory: $REFACTOR_OUTPUT_DIR"
fi

bash "$SCRIPT_DIR/run_evaluation.sh" "$PROMPT_IDX" $FORCE_EVAL $CLEAR_FLAG --output-dir "$REFACTOR_OUTPUT_DIR"

STEP3_END=$(date +%s)
STEP3_DURATION=$((STEP3_END - STEP3_START))

EVAL_STATUS=$?
if [ $EVAL_STATUS -eq 0 ]; then
    echo "✅ Evaluation suite completed"
    echo "⏱️  Step 3 took ${STEP3_DURATION}s ($(( STEP3_DURATION / 60 ))m $(( STEP3_DURATION % 60 ))s)"
else
    echo "⚠️  Evaluation suite report (exit code: $EVAL_STATUS)"
    echo "⏱️  Step 3 took ${STEP3_DURATION}s"
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 4: Generate comparison report
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📈 Step 4: Generate Comparison Report"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
STEP4_START=$(date +%s)

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

STEP4_END=$(date +%s)
STEP4_DURATION=$((STEP4_END - STEP4_START))

COMPARE_STATUS=$?
if [ $COMPARE_STATUS -eq 0 ]; then
    echo "✅ Comparison report generated"
    echo "⏱️  Step 4 took ${STEP4_DURATION}s ($(( STEP4_DURATION / 60 ))m $(( STEP4_DURATION % 60 ))s)"
else
    echo "❌ Comparison report generation failed (exit code: $COMPARE_STATUS)"
    echo "⏱️  Step 4 took ${STEP4_DURATION}s"
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Final summary
# ════════════════════════════════════════════════════════════════════════════

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                     ✅ Workflow Execution Complete!                        ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Calculate total time
END_SECONDS=$(date +%s)
TOTAL_DURATION=$((END_SECONDS - START_SECONDS))

echo "⏱️  TIME SUMMARY:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
[ ! -z "$STEP1_DURATION" ] && echo "  Step 1 (Start Ollama):        ${STEP1_DURATION}s"
[ ! -z "$STEP2_DURATION" ] && echo "  Step 2 (Code Generation):     ${STEP2_DURATION}s ($(( STEP2_DURATION / 60 ))m $(( STEP2_DURATION % 60 ))s) ⭐"
[ ! -z "$STEP3_DURATION" ] && echo "  Step 3 (Evaluation):          ${STEP3_DURATION}s ($(( STEP3_DURATION / 60 ))m $(( STEP3_DURATION % 60 ))s)"
[ ! -z "$STEP4_DURATION" ] && echo "  Step 4 (Comparison Report):   ${STEP4_DURATION}s"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  TOTAL TIME:                   ${TOTAL_DURATION}s ($(( TOTAL_DURATION / 60 ))m $(( TOTAL_DURATION % 60 ))s)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⭐ = If miri/loom was used, Step 2 will be VERY slow"
echo "     - miri: 5-15 min per example (interpreter-based UB detection)"
echo "     - loom: 10-30 min per example (permutation-based concurrency testing)"
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
