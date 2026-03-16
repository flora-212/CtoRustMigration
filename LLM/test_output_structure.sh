#!/bin/bash
# 快速测试新的输出结构
# 使用方式: bash test_output_structure.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🧪 测试新的输出结构"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test 1: Check if output_manager module exists
echo "✅ Test 1: 检查 output_manager 模块"
if [ -f "$SCRIPT_DIR/output_manager.py" ]; then
    echo "   ✓ output_manager.py 存在"
else
    echo "   ✗ output_manager.py 不存在"
    exit 1
fi
echo ""

# Test 2: Check if refractor.py has been updated
echo "✅ Test 2: 检查 refractor.py 是否包含新代码"
if grep -q "from output_manager import OutputManager" "$SCRIPT_DIR/refractor.py"; then
    echo "   ✓ refractor.py 已导入 OutputManager"
else
    echo "   ✗ refractor.py 未导入 OutputManager"
    exit 1
fi
echo ""

# Test 3: Verify Python syntax
echo "✅ Test 3: 检查 Python 语法"
if python3 -m py_compile "$SCRIPT_DIR/output_manager.py" 2>/dev/null; then
    echo "   ✓ output_manager.py 语法正确"
else
    echo "   ✗ output_manager.py 语法错误"
    exit 1
fi

if python3 -m py_compile "$SCRIPT_DIR/refractor.py" 2>/dev/null; then
    echo "   ✓ refractor.py 语法正确"
else
    echo "   ✗ refractor.py 语法错误"
    exit 1
fi
echo ""

# Test 4: Test OutputManager initialization
echo "✅ Test 4: 测试 OutputManager 初始化"
python3 << 'EOF'
import sys
sys.path.insert(0, '/home/guoxy/concrat/LLM')
from output_manager import OutputManager

# Create temp directory for testing
import tempfile
import os

with tempfile.TemporaryDirectory() as tmpdir:
    mgr = OutputManager(result_base_dir=tmpdir)
    output_dir = mgr.initialize(
        prompt_idx=0,
        validate=True,
        strategy="compile",
        max_iterations=3,
        force=False
    )
    
    # Check if directories were created
    assert os.path.exists(output_dir), "Output directory not created"
    assert os.path.exists(os.path.join(output_dir, "rewritten")), "rewritten dir not created"
    assert os.path.exists(os.path.join(output_dir, "examples")), "examples dir not created"
    assert os.path.exists(os.path.join(output_dir, "evaluation")), "evaluation dir not created"
    assert os.path.exists(os.path.join(output_dir, "config.json")), "config.json not created"
    
    print("   ✓ OutputManager 初始化成功")
    print(f"   ✓ 创建的目录结构: {os.path.basename(output_dir)}")
    
    # Test saving methods
    mgr.save_example_rewritten("test_example", "main_rewritten.rs", "test code\n")
    assert os.path.exists(os.path.join(output_dir, "examples/test_example/main_rewritten.rs")), "Example file not saved"
    print("   ✓ save_example_rewritten 可用")
    
    mgr.save_rewritten_file("main_rewritten.rs", "final code\n", is_final=True)
    assert os.path.exists(os.path.join(output_dir, "rewritten/final/main_rewritten.rs")), "Final file not saved"
    print("   ✓ save_rewritten_file 可用")

EOF

if [ $? -ne 0 ]; then
    echo "   ✗ OutputManager 测试失败"
    exit 1
fi
echo ""

# Test 5: Check run.sh was updated
echo "✅ Test 5: 检查 run.sh 是否更新"
if grep -q "result/{timestamp}" "$SCRIPT_DIR/run.sh"; then
    echo "   ✓ run.sh 已更新输出信息"
else
    echo "   ⚠ run.sh 输出信息可能未完全更新"
fi
echo ""

# Test 6: Check run_evaluation.sh was updated
echo "✅ Test 6: 检查 run_evaluation.sh 是否更新"
if grep -q "timestamped" "$SCRIPT_DIR/run_evaluation.sh"; then
    echo "   ✓ run_evaluation.sh 已支持时间戳目录"
else
    echo "   ✗ run_evaluation.sh 未能支持时间戳目录"
    exit 1
fi
echo ""

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║                    ✅ 所有测试通过！                                       ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "📝 新的输出结构已准备好！"
echo ""
echo "下一步："
echo "  1. 运行: ./run.sh 0 --validate --strategy compile"
echo "  2. 等待生成完成"
echo "  3. 查看结果: ls result/"
echo ""
