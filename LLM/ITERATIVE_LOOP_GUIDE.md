# 迭代生成-验证-反馈闭环系统

## 概述

这个系统实现了 LLM 驱动的代码转换的完整迭代闭环：
1. **生成阶段** (refractor.py): LLM 根据系统提示词生成、重写代码
2. **验证阶段** (validator.py): 对生成的代码进行多种验证检查
3. **反馈阶段**: 如果验证失败，将错误信息反馈给 LLM，继续迭代

## 文件结构

```
LLM/
├── refractor.py          # 代码生成脚本（已升级，支持迭代）
├── validator.py          # 验证器模块（新增）
├── compare_all.py        # 对比评估工具（原有）
├── evaluation/           # 评估结果输出目录
└── result/              # 缓存和报告目录
```

## 快速开始

### 1. 基础生成（无验证）

```bash
cd /home/guoxy/concrat/LLM

# 使用 SYSTEM_PROMPT_0 生成
python3 refractor.py 0

# 使用 SYSTEM_PROMPT_1 生成
python3 refractor.py 1

# 使用 SYSTEM_PROMPT_2 生成
python3 refractor.py 2

# 强制重新生成
python3 refractor.py 0 --force
```

### 2. 迭代验证生成（新增）

启用 `--validate` 标志来激活迭代闭环。生成后立即验证，如果失败则继续迭代。

```bash
# 使用编译验证进行迭代（默认）
python3 refractor.py 1 --validate

# 使用全面验证进行迭代
python3 refractor.py 1 --validate --strategy comprehensive

# 自定义最大迭代次数（默认 3）
python3 refractor.py 1 --validate --max-iterations 5

# 组合选项
python3 refractor.py 1 --force --validate --strategy safety --max-iterations 4
```

### 3. 验证策略

支持四种验证策略：

| 策略 | 说明 | 用途 |
|------|------|------|
| `compile` | 检查代码是否编译成功 | 基础检查，最快速 |
| `safety` | 检查 unsafe、pthread 等不安全模式 | 安全性评估 |
| `lock_safety` | 分析锁的使用模式 | 并发代码质量 |
| `comprehensive` | 上述所有检查 | 全面评估 |

```bash
# 仅编译检查
python3 refractor.py 1 --validate --strategy compile

# 安全检查
python3 refractor.py 1 --validate --strategy safety

# 锁安全分析
python3 refractor.py 1 --validate --strategy lock_safety

# 全面检查
python3 refractor.py 1 --validate --strategy comprehensive
```

## 系统工作流程

### 无验证模式（原有）
```
Original Code → LLM Rewrite → Output File
                    ↓
              save main_rewritten_N.rs
```

### 有验证模式（新增）
```
Original Code → LLM Rewrite ──┐
                              ↓
                         Validation
                          /      \
                    PASS /        \ FAIL
                       /            \
                   Save              Feedback
                    ↓                  ↑
           main_rewritten_N.rs    Errors & Metrics
```

## 验证流程详解

### 编译验证 (compile)
- 尝试编译代码
- 若失败，提取错误信息
- 将错误反馈给 LLM 进行修复

### 安全检查 (safety)
检查以下不安全模式：
- `unsafe` 关键字数量
- `pthread_*` 调用数
- 原始指针（`*mut`, `*const`）
- `static mut` 变量
- `libc::` 调用

### 锁安全分析 (lock_safety)
检查并发代码质量：
- 是否仍有 pthread_mutex（应被 std::sync::Mutex 替代）
- Arc<Mutex<T>> 使用是否正确
- thread::spawn 是否有正确的 join()
- 检测潜在的死锁风险

### 全面检查 (comprehensive)
执行上述所有检查。

## 迭代过程

当启用 `--validate` 时，系统会：

1. **第一次生成**: LLM 根据原始代码和系统提示词生成代码
2. **第一次验证**: 对生成的代码进行验证
   - 若通过 ✅：保存输出，继续下一个文件
   - 若失败 ❌：进入迭代
3. **迭代反馈**: 
   - 提取验证失败的具体信息
   - 将错误消息作为上下文反馈给 LLM
   - LLM 基于反馈进行修复
4. **重复验证**: 重复步骤 2，直到通过或达到最大迭代次数

### 变量追踪

迭代过程中维护对话历史：
```
messages = [
    {"role": "system", "content": system_prompt},
    {"role": "user", "content": "Rewrite the following code: ..."},
    {"role": "assistant", "content": rewritten_code},
    {"role": "assistant", "content": "Validation Feedback: ..."},  # 如果失败
    ...
]
```

这使 LLM 能够理解之前的尝试和失败原因。

## 可配置参数

### refractor.py 中的全局配置

编辑 refractor.py 顶部的配置：

```python
MODEL = "qwen2.5-coder:14b"      # LLM 模型
MAX_RETRIES = 3                   # API 调用重试次数
RETRY_DELAY = 5                   # 重试延迟（秒）
VALIDATION_STRATEGY = ...         # 默认验证策略
```

### 命令行参数

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `prompt_idx` | 系统提示词索引 (0/1/2) | 0 |
| `--force` | 覆盖已存在的文件 | false |
| `--validate` | 启用迭代验证 | false |
| `--strategy` | 验证策略 | compile |
| `--max-iterations` | 最大迭代次数 | 3 |

## 输出文件

生成的文件保存为：
```
examples/
├── struct_init/
│   ├── main.rs                        # 原始 C2Rust 代码
│   ├── main_rewritten_0.rs           # 使用 SYSTEM_PROMPT_0 生成
│   ├── main_rewritten_1.rs           # 使用 SYSTEM_PROMPT_1 生成
│   └── main_rewritten_2.rs           # 使用 SYSTEM_PROMPT_2 生成
└── ...
```

## 验证器的独立使用

validator.py 也可以独立使用：

```bash
# 验证单个文件（编译）
python3 validator.py /path/to/code.rs

# 全面验证
python3 validator.py /path/to/code.rs comprehensive /path/to/example_dir

# 安全检查
python3 validator.py /path/to/code.rs safety
```

## 代码示例

### Python API 使用

```python
from validator import CodeValidator, ValidationStrategy

# 创建验证器
validator = CodeValidator()

# 读取代码
with open("main_rewritten_1.rs") as f:
    code = f.read()

# 执行全面验证
passed, report, results = validator.validate_and_report(
    "main_rewritten_1.rs",
    code,
    ValidationStrategy.COMPREHENSIVE,
    example_dir="/path/to/example"
)

# 输出结果
print(f"Passed: {passed}")
print(f"Report:\n{report}")
for result in results:
    print(f"  {result}")
```

### 系统提示词定制

可在 refractor.py 中添加新的系统提示词：

```python
SYSTEM_PROMPT_3 = """You are a Rust expert...
Requirements:
1. ...
2. ...
"""
```

然后使用：
```bash
python3 refractor.py 3 --validate
```

## 工作流示例

### 场景：使用锁安全验证进行迭代优化

```bash
# 1. 生成初始版本（带迭代）
python3 refractor.py 1 --validate --strategy lock_safety --max-iterations 5

# 2. 查看结果
ls -la examples/*/main_rewritten_1.rs

# 3. 对比评估
python3 compare_all.py 1

# 4. 查看对比报告
cat LLM/result/1/comparison_report.json
```

## 故障排除

### 问题：验证超时
**症状**: 编译检查花费很长时间或超时
**解决**: 
- 增加超时时间（validator.py 中修改 timeout 参数）
- 减少最大迭代次数
- 使用更快的验证策略（compile 而非 comprehensive）

### 问题：LLM 无法修复
**症状**: 多次迭代后仍无法编译
**可能原因**:
- 系统提示词不够清晰
- 代码结构复杂
- LLM 模型能力限制

**解决**:
- 调整系统提示词
- 减少 max_iterations，保存部分结果
- 检查 validator 是否过于严格

### 问题：验证器报错
**症状**: "Cargo.toml not found" 或编译错误
**解决**:
- 确保 example_dir 正确传递
- 检查 Cargo.toml 是否存在
- 查看依赖是否已下载

## 性能优化建议

1. **策略选择**: 
   - 快速迭代：用 `compile`
   - 质量检查：用 `comprehensive`

2. **迭代次数**:
   - 简单修改：3-4 次
   - 复杂转换：5-6 次

3. **并行化**:
   - 目前是顺序处理，可在 main 中添加线程池

4. **缓存**:
   - validator 自动缓存 concrat 结果
   - 可跳过已验证的文件

## 扩展方向

1. **新的验证策略**: 在 ValidationStrategy 和 CodeValidator 中添加
2. **多模型支持**: 修改 MODEL 变量或添加模型选择参数
3. **自定义反馈**: 修改 rewrite_file_with_validation 中的反馈生成逻辑
4. **启发式修复**: 添加常见错误的自动修复模式

## 参考文献

- 原始对比工具：[compare_all.py](compare_all.py)
- 验证器详细代码：[validator.py](validator.py)
- 生成器详细代码：[refractor.py](refractor.py)
