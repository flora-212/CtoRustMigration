#!/bin/bash
# 迭代生成-验证-反馈闭环系统演示脚本

set -e

cd /home/guoxy/concrat/LLM

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║     迭代生成-验证-反馈闭环系统演示                                         ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# 检查依赖
echo "[1/4] 检查依赖..."
if ! command -v python3 &> /dev/null; then
    echo "❌ 错误: 未找到 python3"
    exit 1
fi

if ! python3 -c "import ollama" 2>/dev/null; then
    echo "❌ 错误: ollama 包未安装"
    echo "   运行: pip install ollama"
    exit 1
fi

echo "✅ 依赖检查通过"
echo ""

# 显示可用的验证器模块
echo "[2/4] 验证器模块检查..."
if [ -f "validator.py" ]; then
    echo "✅ validator.py 已创建"
    python3 -c "from validator import CodeValidator, ValidationStrategy; print('   可用策略:', ', '.join([s.value for s in ValidationStrategy]))"
else
    echo "❌ validator.py 未找到"
    exit 1
fi
echo ""

# 显示使用方式
echo "[3/4] 显示使用示例..."
echo ""
echo "── 基础生成（无验证）────────────────────────────────────────"
echo "  python3 refractor.py 1"
echo ""
echo "── 迭代验证生成────────────────────────────────────────────"
echo "  # 编译验证（最快）"
echo "  python3 refractor.py 1 --validate"
echo ""
echo "  # 安全性验证"
echo "  python3 refractor.py 1 --validate --strategy safety"
echo ""
echo "  # 锁安全分析"
echo "  python3 refractor.py 1 --validate --strategy lock_safety"
echo ""
echo "  # 全面检查"
echo "  python3 refractor.py 1 --validate --strategy comprehensive"
echo ""
echo "  # 自定义迭代次数"
echo "  python3 refractor.py 1 --validate --max-iterations 5"
echo ""
echo "── 验证单个文件────────────────────────────────────────────"
echo "  python3 validator.py ../examples/struct_init/main.rs compile"
echo ""
echo "── 对比评估────────────────────────────────────────────────"
echo "  python3 compare_all.py 1"
echo ""

# 系统架构图
echo "[4/4] 系统架构"
echo ""
cat << 'EOF'
┌─────────────────────────────────────────────────────────────────────┐
│                  迭代生成-验证-反馈闭环                              │
└─────────────────────────────────────────────────────────────────────┘

  原始C2Rust代码
         │
         ▼
    ┌─────────────┐
    │ 1. Refactor │◄────────────────────────────────┐
    │  (LLM生成)  │                                  │
    └────┬────────┘                                  │
         │                                            │
         ▼                                            │
    ┌─────────────────┐     ❌ (验证失败)             │
    │ 2. Validate     ├──────────────────►  反馈错误  │
    │   (编译/安全)   │                    给LLM     │
    └────┬────────────┘                              │
         │                                            │
         ✅ (验证通过)                                 │
         │                                            │
         ▼                                            │
    ┌──────────────────┐                             │
    │ 3. Save Output   │                             │
    │ main_rewritten_N │                             │
    └────┬─────────────┘      迭代重试(最多N次)◄─────┘
         │
         ▼
    输出文件

典型迭代过程:

第1次: 代码生成 → 验证 ❌ (编译错误: missing type)
第2次: 考虑错误提示 → 代码生成 → 验证 ❌ (safety: 3 unsafe blocks)
第3次: 优化安全性 → 代码生成 → 验证 ✅ (通过!)
第4次: 保存输出

验证策略:
  ┌─────────┬─────────────┬──────────────┐
  │compile  │safety       │lock_safety   │
  ├─────────┼─────────────┼──────────────┤
  │最快速   │检查 unsafe  │分析锁使用    │
  │检查编译 │pthread 调用 │线程join()    │
  │         │raw 指针     │死锁风险      │
  └─────────┴─────────────┴──────────────┘
EOF
echo ""

# 显示配置
echo "═══════════════════════════════════════════════════════════════════"
echo "配置信息"
echo "═══════════════════════════════════════════════════════════════════"
python3 << 'PYEOF'
from refractor import MODEL, MAX_RETRIES, RETRY_DELAY
from validator import CodeValidator

print(f"LLM 模型: {MODEL}")
print(f"API 调用重试: {MAX_RETRIES} 次")
print(f"重试延迟: {RETRY_DELAY} 秒")
print(f"验证器版本: v1.0 (CodeValidator)")
PYEOF

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "✅ 演示脚本完成!"
echo ""
echo "下一步: 运行以下命令开始迭代生成"
echo ""
echo "  cd /home/guoxy/concrat/LLM"
echo "  python3 refractor.py 1 --validate --strategy compile --max-iterations 3"
echo ""
echo "或查看详细文档:"
echo "  cat ITERATIVE_LOOP_GUIDE.md"
echo "═══════════════════════════════════════════════════════════════════"
