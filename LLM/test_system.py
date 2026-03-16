#!/usr/bin/env python3
"""
快速测试迭代闭环系统 - 验证所有组件是否正常工作
"""

import sys
import os

def test_imports():
    """测试所有必要的模块导入"""
    print("=" * 70)
    print("测试 1: 模块导入")
    print("=" * 70)
    
    try:
        import ollama
        print("✅ ollama: 已安装")
    except ImportError:
        print("❌ ollama: 未安装 (pip install ollama)")
        return False
    
    try:
        from validator import CodeValidator, ValidationStrategy
        print("✅ validator: 已加载")
    except ImportError as e:
        print(f"❌ validator: {e}")
        return False
    
    return True

def test_validator():
    """测试验证器功能"""
    print("\n" + "=" * 70)
    print("测试 2: 验证器基本功能")
    print("=" * 70)
    
    try:
        from validator import CodeValidator, ValidationStrategy
        
        # 创建验证器实例
        validator = CodeValidator()
        print("✅ CodeValidator 实例创建成功")
        
        # 测试安全指标提取
        test_code = '''
fn main() {
    unsafe {
        let x = &mut 5;
    }
}
        '''
        
        metrics = validator.safety_metrics(test_code)
        print(f"✅ 安全指标提取: {metrics['unsafe']} unsafe 块")
        
        # 测试锁安全分析
        analysis = validator.analyze_lock_safety(test_code, "test")
        print(f"✅ 锁安全分析: {len(analysis['issues'])} 个问题")
        
        print("\n可用的验证策略:")
        for strategy in ValidationStrategy:
            print(f"  - {strategy.value}")
        
        return True
    except Exception as e:
        print(f"❌ 验证器测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_refractor():
    """测试生成器配置"""
    print("\n" + "=" * 70)
    print("测试 3: 生成器配置")
    print("=" * 70)
    
    try:
        import refractor
        
        print(f"✅ 模型: {refractor.MODEL}")
        print(f"✅ API 重试次数: {refractor.MAX_RETRIES}")
        print(f"✅ 重试延迟: {refractor.RETRY_DELAY}s")
        
        print(f"\n可用的系统提示词:")
        for idx in sorted(refractor.SYSTEM_PROMPTS.keys()):
            prompt_preview = refractor.SYSTEM_PROMPTS[idx][:60] + "..."
            print(f"  - SYSTEM_PROMPT_{idx}: {prompt_preview}")
        
        # 测试新函数是否存在
        if hasattr(refractor, 'rewrite_file_with_validation'):
            print(f"\n✅ 迭代验证函数已加载: rewrite_file_with_validation()")
        else:
            print(f"\n❌ 迭代验证函数不存在")
            return False
        
        return True
    except Exception as e:
        print(f"❌ 生成器配置测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_example_file():
    """测试示例文件是否存在"""
    print("\n" + "=" * 70)
    print("测试 4: 示例文件检查")
    print("=" * 70)
    
    examples_dir = "/home/guoxy/concrat/examples"
    
    if not os.path.exists(examples_dir):
        print(f"❌ 示例目录不存在: {examples_dir}")
        return False
    
    import glob
    examples = sorted(glob.glob(f"{examples_dir}/*/main.rs"))
    
    if not examples:
        print(f"❌ 在 {examples_dir} 中找不到任何示例")
        return False
    
    print(f"✅ 找到 {len(examples)} 个示例文件:")
    for i, ex in enumerate(examples[:5], 1):
        ex_name = os.path.basename(os.path.dirname(ex))
        print(f"   {i}. {ex_name}")
    
    if len(examples) > 5:
        print(f"   ... 和 {len(examples) - 5} 个更多的文件")
    
    return True

def main():
    print("\n" + "╔" + "=" * 68 + "╗")
    print("║" + " 迭代生成-验证-反馈闭环系统 - 系统检查 ".center(68) + "║")
    print("╚" + "=" * 68 + "╝")
    
    tests = [
        ("模块导入", test_imports),
        ("验证器", test_validator),
        ("生成器", test_refractor),
        ("示例文件", test_example_file),
    ]
    
    results = []
    for name, test_func in tests:
        try:
            result = test_func()
            results.append((name, result))
        except Exception as e:
            print(f"\n❌ {name} 测试异常: {e}")
            results.append((name, False))
    
    # 总结
    print("\n" + "=" * 70)
    print("系统检查总结")
    print("=" * 70)
    
    passed = sum(1 for _, r in results if r)
    total = len(results)
    
    for name, result in results:
        status = "✅" if result else "❌"
        print(f"{status} {name}")
    
    print(f"\n总体: {passed}/{total} 个检查通过")
    
    if passed == total:
        print("\n✅ 系统已准备就绪!")
        print("\n快速开始:")
        print("  cd /home/guoxy/concrat/LLM")
        print("  python3 refractor.py 1 --validate --strategy compile --max-iterations 3")
        return 0
    else:
        print("\n⚠️  系统检查未完全通过，请检查上述错误")
        return 1

if __name__ == "__main__":
    sys.exit(main())
