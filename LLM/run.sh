#!/bin/bash
# 迭代生成-验证-评估完整流程脚本
#
# 使用方式:
#   ./run.sh                                          # 使用默认参数 (prompt=1, validate=true)
#   ./run.sh 0                                        # 使用 SYSTEM_PROMPT_0
#   ./run.sh 1 --validate                             # 显式启用验证（默认已启用）
#   ./run.sh 1 --validate --strategy comprehensive     # 全面验证
#   ./run.sh 1 --validate --max-iterations 5           # 自定义迭代次数
#   ./run.sh 1 --force                                # 强制重新生成和评估
#   ./run.sh 1 --force --strategy lock_safety         # 强制 + 锁安全策略
#   ./run.sh 1 --no-validate                          # 关闭验证循环

set -e

# ════════════════════════════════════════════════════════════════════════════
# 参数解析
# ════════════════════════════════════════════════════════════════════════════

# 默认值
PROMPT_IDX=1                          # 默认改为 prompt 1
VALIDATE_FLAG="--validate"            # 默认启用验证
STRATEGY="compile"                    # 验证策略
MAX_ITERATIONS=3                      # 最大迭代次数
FORCE_REWRITE=""
FORCE_EVAL=""
FORCE_GENERATE=""
CLEAR_FLAG=""
VERBOSE=""

# 先检查是否有 --help
for arg in "$@"; do
    if [ "$arg" = "--help" ]; then
        echo "迭代生成-验证-评估完整流程脚本"
        echo ""
        echo "使用方式:"
        echo "  ./run.sh                                    # 默认: prompt=1, validate=true"
        echo "  ./run.sh 0                                  # 使用 SYSTEM_PROMPT_0"
        echo "  ./run.sh 1 --validate                       # 启用验证"
        echo "  ./run.sh 1 --no-validate                    # 禁用验证"
        echo "  ./run.sh 1 --strategy {compile|safety|...}  # 验证策略"
        echo "  ./run.sh 1 --max-iterations 5               # 迭代次数"
        echo "  ./run.sh 1 --force                          # 强制重新执行"
        echo "  ./run.sh 1 --verbose                        # 详细输出"
        echo ""
        echo "策略:"
        echo "  compile        - 编译检查（最快）"
        echo "  safety         - 安全性检查"
        echo "  lock_safety    - 锁安全分析"
        echo "  comprehensive  - 全面检查"
        exit 0
    fi
done

# 处理第一个位置参数 (prompt_idx)
if [ $# -gt 0 ] && [ "$1" != "--"* ]; then
    PROMPT_IDX="$1"
    shift
fi

# 处理所有标志
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
                echo "❌ 错误: --strategy 需要参数"
                exit 1
            fi
            STRATEGY="$2"
            shift
            ;;
        --max-iterations)
            if [ -z "$2" ]; then
                echo "❌ 错误: --max-iterations 需要参数"
                exit 1
            fi
            MAX_ITERATIONS="$2"
            shift
            ;;
        --verbose)
            VERBOSE="--verbose"
            ;;
        *)
            echo "❌ 未知参数: $1"
            exit 1
            ;;
    esac
    shift
done

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# ════════════════════════════════════════════════════════════════════════════
# 欢迎横幅
# ════════════════════════════════════════════════════════════════════════════

echo ""
echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                  迭代生成-验证-评估完整流程脚本                            ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "✅ 配置信息:"
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
# Step 1: 启动 Ollama
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📡 Step 1: 启动 Ollama LLM 服务"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if pgrep -x ollama > /dev/null 2>&1; then
    echo "✅ Ollama 已在运行"
else
    echo "🚀 启动 Ollama..."
    ollama serve > /dev/null 2>&1 &
    OLLAMA_PID=$!
    echo "   Ollama PID: $OLLAMA_PID"
    echo "   等待 Ollama 就绪..."
    for i in $(seq 1 30); do
        if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
            echo "✅ Ollama 已就绪"
            break
        fi
        sleep 1
        if [ $i -eq 30 ]; then
            echo "❌ Ollama 启动超时"
            exit 1
        fi
    done
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 2: 代码生成 + 迭代验证
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔄 Step 2: 生成代码 (SYSTEM_PROMPT_${PROMPT_IDX})"
if [ ! -z "$VALIDATE_FLAG" ]; then
    echo "   + 迭代验证循环 (策略: $STRATEGY, 最大迭代: $MAX_ITERATIONS)"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ ! -z "$VALIDATE_FLAG" ]; then
    # 使用验证循环
    python3 "$SCRIPT_DIR/refractor.py" "$PROMPT_IDX" \
        $VALIDATE_FLAG \
        --strategy "$STRATEGY" \
        --max-iterations "$MAX_ITERATIONS" \
        $FORCE_REWRITE \
        $VERBOSE
else
    # 无验证循环
    python3 "$SCRIPT_DIR/refractor.py" "$PROMPT_IDX" $FORCE_REWRITE $VERBOSE
fi

REFACTOR_STATUS=$?
if [ $REFACTOR_STATUS -eq 0 ]; then
    echo "✅ 代码生成完成"
else
    echo "❌ 代码生成失败 (状态码: $REFACTOR_STATUS)"
    exit $REFACTOR_STATUS
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 3: 运行评估套件
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Step 3: 运行评估套件"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

bash "$SCRIPT_DIR/run_evaluation.sh" "$PROMPT_IDX" $FORCE_EVAL $CLEAR_FLAG

EVAL_STATUS=$?
if [ $EVAL_STATUS -eq 0 ]; then
    echo "✅ 评估套件完成"
else
    echo "⚠️  评估套件报告 (状态码: $EVAL_STATUS)"
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# Step 4: 生成对比报告
# ════════════════════════════════════════════════════════════════════════════

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📈 Step 4: 生成对比报告"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

python3 "$SCRIPT_DIR/evaluation/generate_comparison.py" "$PROMPT_IDX" $FORCE_GENERATE

COMPARE_STATUS=$?
if [ $COMPARE_STATUS -eq 0 ]; then
    echo "✅ 对比报告生成完成"
else
    echo "❌ 对比报告生成失败 (状态码: $COMPARE_STATUS)"
fi

echo ""

# ════════════════════════════════════════════════════════════════════════════
# 最终总结
# ════════════════════════════════════════════════════════════════════════════

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                       ✅ 流程执行完成！                                    ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "📁 输出文件位置:"
echo "   - 生成的代码: examples/*/main_rewritten_${PROMPT_IDX}.rs"
echo "   - 对比报告: result/${PROMPT_IDX}/comparison_report.json"
echo "   - 评估结果: result/${PROMPT_IDX}/"
echo ""
echo "🔍 查看结果:"
echo "   # 查看对比报告"
echo "   cat result/${PROMPT_IDX}/comparison_report.json | jq ."
echo ""
echo "   # 查看评估详情"
echo "   cat result/${PROMPT_IDX}/*.json"
echo ""
echo "🧹 清理生成的文件:"
echo "   # 查看所有生成的文件"
echo "   python3 cleanup.py --all --dry-run"
echo ""
echo "   # 删除所有版本"
echo "   python3 cleanup.py --all --yes"
echo ""
