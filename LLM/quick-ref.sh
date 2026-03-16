#!/bin/bash
# 快速参考卡片

cat << 'EOF'
╔════════════════════════════════════════════════════════════════════════════╗
║                  🚀 迭代生成-验证-评估闭环系统                            ║
║                        快速命令参考                                        ║
╚════════════════════════════════════════════════════════════════════════════╝

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 📋 完整流程 (自动 Ollama + 生成 + 验证 + 评估)                            ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  ./run.sh                  # 默认: prompt=1, 启用验证, compile 策略
  ./run.sh 0                # 使用 SYSTEM_PROMPT_0
  ./run.sh 1 --validate     # 显式启用验证

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 🔧 验证策略选择                                                           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  速度优先 (最快):
  ./run.sh 1 --validate --strategy compile
  
  安全优先:
  ./run.sh 1 --validate --strategy safety
  
  并发代码优化:
  ./run.sh 1 --validate --strategy lock_safety
  
  质量优先 (最严格):
  ./run.sh 1 --validate --strategy comprehensive --max-iterations 5

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ ⚙️  迭代参数调整                                                          ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  增加迭代次数 (更好的结果, 更慢):
  ./run.sh 1 --validate --max-iterations 5
  
  减少迭代次数 (更快):
  ./run.sh 1 --validate --max-iterations 1

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 🔄 强制重新执行                                                           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  重新生成代码:
  ./run.sh 1 --force --validate
  
  仅重新生成代码:
  ./run.sh 1 --force-rewrite --validate
  
  仅重新评估:
  ./run.sh 1 --force-eval

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 🧪 单独工具使用                                                           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  仅生成 (无验证):
  python3 refractor.py 1
  
  生成 + 验证:
  python3 refractor.py 1 --validate --strategy comprehensive
  
  验证单个文件:
  python3 validator.py examples/struct_init/main_rewritten_1.rs comprehensive
  
  对比评估:
  python3 compare_all.py 1
  
  清理文件:
  python3 cleanup.py --all --dry-run     # 预览
  python3 cleanup.py --keep 1 --yes      # 保留版本 1

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 📊 查看结果                                                               ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  对比报告:
  cat result/1/comparison_report.json | jq .
  
  查看生成的代码:
  cat examples/struct_init/main_rewritten_1.rs
  
  对比原始版本:
  diff examples/struct_init/main.rs examples/struct_init/main_rewritten_1.rs

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 📚 详细文档                                                               ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  run.sh 详细指南:
  cat RUN_SCRIPT_GUIDE.md | less
  
  迭代闭环系统指南:
  cat ITERATIVE_LOOP_GUIDE.md | less
  
  清理脚本指南:
  cat CLEANUP_GUIDE.md | less
  
  实现细节:
  cat IMPLEMENTATION_SUMMARY.md | less

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 🎯 常见工作流                                                             ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  工作流 1: 快速实验
  ─────────────────────
  1. ./run.sh 1 --validate --strategy compile
  2. python3 compare_all.py 1
  3. cat result/1/comparison_report.json | jq .
  
  工作流 2: 质量优先
  ─────────────────────
  1. ./run.sh 1 --validate --strategy comprehensive --max-iterations 5
  2. python3 cleanup.py --keep 1 --yes
  3. python3 compare_all.py 1
  
  工作流 3: 版本对比
  ─────────────────────
  1. ./run.sh 0 --force
  2. ./run.sh 1 --force
  3. python3 compare_all.py 0
  4. python3 compare_all.py 1
  5. diff result/0/comparison_report.json result/1/comparison_report.json

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 🔍 验证反馈看例子                                                         ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

  第 1 次迭代失败 (编译错误):
  ─────────────────────────
  [Iteration 1/3] ❌ 验证失败
    📋 验证反馈详情:
    ❌ [compile] Compilation failed:
       └─ error[E0412]: cannot find type `T` in this scope
    🔄 向 LLM 反馈错误信息...
  
  第 2 次迭代失败 (安全问题):
  ─────────────────────────
  [Iteration 2/3] ❌ 验证失败
    📋 验证反馈详情:
    ❌ [safety] Safety concerns: 2 unsafe block(s)
    🔄 向 LLM 反馈错误信息...
  
  第 3 次迭代通过:
  ─────────────────────────
  [Iteration 3/3] ✅ 验证通过！
    ✅ [compile] Code compiles successfully
    ✅ [safety] No major safety concerns

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 提示:
  • 默认 prompt 已改为 1 (更强大的提示词)
  • 默认启用验证循环，看不到 --validate 时仍在运行
  • 验证器每一轮的错误都会显示，可以追踪改进过程
  • 使用 --verbose 获得更详细的输出

❓ 帮助:
  ./run.sh --help
  python3 cleanup.py --help
  python3 validator.py --help

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
EOF
