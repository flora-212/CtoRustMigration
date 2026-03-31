# 快速使用指南 - 负样本 + 参数化 Prompt 系统

## 基本命令

### 1️⃣ 仅处理正样本（传统方式）
```bash
./run.sh 6 --validate --strategy compile
```
- 从 `/home/guoxy/concrat/examples/` 读取 ✓
- 结果保存到 `result/TIMESTAMP_6_compile/`

### 2️⃣ 处理正样本 + 负样本
```bash
./run.sh 6 --validate --strategy compile --include-negative
```
- 从 `examples/` 和 `examples_negative/` 读取 ✓
- 所有结果混合保存到同一输出目录
- 评估时自动识别样本类型

### 3️⃣ 仅处理负样本
```bash
./run.sh 6 --validate --strategy compile --negative-only
```
- 从 `examples_negative/` 读取 ✓
- 用于测试模型在"已知困难代码"上的表现

---

## 参数化 Prompt 系统

### 当前支持的 Prompt 索引

```python
# 在 refractor.py 中使用：
# SYSTEM_PROMPT_0 到 SYSTEM_PROMPT_6 等

./run.sh 0  # 使用 SYSTEM_PROMPT_0 + 默认 FIXING_PROMPT
./run.sh 3  # 使用 SYSTEM_PROMPT_3 + 默认 FIXING_PROMPT
./run.sh 5  # 使用 SYSTEM_PROMPT_5 + 默认 FIXING_PROMPT
```

### 添加新 Prompt 组合（示例）

#### 步骤 1: 在 refractor.py 中定义新 prompt

```python
# 在 SYSTEM_PROMPT_6 之后添加
SYSTEM_PROMPT_7 = """您的新 prompt 文本..."""

# 如果需要特殊的 fixing_prompt
FIXING_PROMPT_CUSTOM = """修复 prompt 文本..."""
```

#### 步骤 2: 注册到 PROMPT_CONFIGS（自动或手动）

```python
# 自动注册（SYSTEM_PROMPT 会自动被识别）
# 或手动覆盖：
PROMPT_CONFIGS[7] = {
    "system_prompt": SYSTEM_PROMPT_7,
    "fixing_prompt": FIXING_PROMPT_CUSTOM,  # 如果需要特殊 fixing_prompt
}
```

#### 步骤 3: 使用新 Prompt

```bash
./run.sh 7 --validate --strategy compile
```

---

## 输出目录结构

```
result/
├── 20260329_135246_6_compile/              # 时间戳目录
│   ├── config.json                         # 配置信息
│   ├── examples/
│   │   ├── array_const/
│   │   │   ├── round1.rs, round2.rs, ...   # 迭代历史
│   │   │   ├── final.rs                    # 最终版本
│   │   │   ├── rounds_metadata.json        # 所有轮的元数据
│   │   │   └── conversation_history.json   # 与 LLM 的完整对话
│   │   ├── array_simple/
│   │   │   └── ...
│   │   └── ... (其他正样本)
│   ├── evaluation/
│   │   ├── comparison_report.json          # 评估报告
│   │   └── comparison_report.md
│   └── (negative samples，如使用 --include-negative)
│       ├── examples/
│       │   ├── array_const_____deadlock/
│       │   │   ├── final.rs
│       │   │   └── ...
│       │   └── ...
```

---

## 评估报告示例

### comparison_report.json 中标记

```json
{
  "name": "array_simple",
  "original": { ... },
  "concrat": { ... },
  "llm": { ... },
  "is_negative": false  // ← 标记正/负样本
}
```

### 控制台输出示例

```
[POS] array_const      │ original  │ (base)   │ 7 │ 25 │ 52 │ 2 │ 99 │ 0 │ 160
      │ concrat   │ ✅       │ 7 │ 22 │ 32 │ 1 │ 54 │ 7 │ 121
      │ LLM       │ ✅       │ 6 │ 0  │ 2  │ 1 │ 3  │ 7 │ 67

[NEG] array_const____deadlock │ original │ (base) │ ... │ 160
      │ concrat                │ ✅     │ ...
      │ LLM                    │ ❌     │ (compilation errors)
```

---

## 常见场景

### 场景 1: 测试新 Prompt 对正样本的效果
```bash
./run.sh 7 --validate --strategy compile --force
```

### 场景 2: 评估对负样本的抗性（容错能力）
```bash
./run.sh 6 --validate --strategy compile --negative-only --force
```

### 场景 3: 综合评估（正+负）
```bash
./run.sh 6 --validate --strategy compile --include-negative --force
```

### 场景 4: 快速测试（无验证循环）
```bash
./run.sh 6 --no-validate --include-negative
```

---

## 查看结果

### 获取最新运行的输出目录
```bash
cat .last_refactor_output
# 输出: /home/guoxy/concrat/LLM/result/20260329_135246_6_compile
```

### 查看评估报告
```bash
# JSON 格式
cat result/20260329_135246_6_compile/evaluation/comparison_report.json | jq .

# Markdown 格式（可读性更好）
cat result/20260329_135246_6_compile/evaluation/comparison_report.md
```

### 查看特定示例的详情
```bash
# 查看负样本的转换尝试
cat result/20260329_135246_6_compile/examples/array_const____deadlock/final.rs

# 查看完整的 LLM 对话
cat result/20260329_135246_6_compile/examples/array_const____deadlock/conversation_history.json | jq .
```

---

## 故障排查

### 问题 1: 找不到 examples_negative 目录
```bash
# 检查目录是否存在
ls -la /home/guoxy/concrat/examples_negative/

# 如果不存在，创建测试用例或跳过负样本
./run.sh 6 --validate --strategy compile  # 仅正样本
```

### 问题 2: Prompt 配置失败
```
Error: Prompt config 999 not found. Available: [0, 1, 2, 3, 4, 5, 6]
```
**解决**: 使用可用的 prompt_idx，例如 0-6

### 问题 3: 编译环境差异
- 参见 [COMPILE_MISMATCH_ANALYSIS.md](COMPILE_MISMATCH_ANALYSIS.md)
- LLM 代码在 refractor.py 编译成功，但在 evaluation 中失败
- 原因：使用了不同的 example_dir（已在本版本修复）

---

## 性能提示

### 加快速度
```bash
# 1. 仅运行正样本（更快）
./run.sh 6 --validate --strategy compile

# 2. 减少迭代次数
./run.sh 6 --validate --strategy compile --max-iterations 3

# 3. 无验证模式（仅一次编译）
./run.sh 6 --no-validate
```

### 全面评估（但更慢）
```bash
# 所有示例，完整验证
./run.sh 6 --validate --strategy comprehensive --include-negative --force
```

---

## 相关文档

- 📖 **详细实现文档**: [NEGATIVE_SAMPLES_AND_PROMPT_CONFIG.md](NEGATIVE_SAMPLES_AND_PROMPT_CONFIG.md)
- 🔍 **编译不一致分析**: [COMPILE_MISMATCH_ANALYSIS.md](COMPILE_MISMATCH_ANALYSIS.md)
- 📝 **主 README**: [README.md](README.md)

---

## 快速参考表

| 需求 | 命令 |
|------|------|
| 测试单个 prompt | `./run.sh 5 --validate --strategy compile` |
| 包括负样本 | `./run.sh 5 --validate --strategy compile --include-negative` |
| 只用负样本 | `./run.sh 5 --validate --strategy compile --negative-only` |
| 快速测试 | `./run.sh 5 --no-validate` |
| 强制重新运行 | `./run.sh 5 --validate --force` |
| 完整评估 | `./run.sh 5 --validate --strategy comprehensive --include-negative --force` |
