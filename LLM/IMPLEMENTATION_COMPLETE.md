# 实现总结 - 负样本支持与参数化 Prompt 系统

## ✅ 完成的工作

### 1. run.sh 改动 ✓
- 添加 `--include-negative` 参数
- 添加 `--negative-only` 参数
- 更新帮助信息
- 传递新参数给 refractor.py

### 2. refractor.py 改动 ✓
- 创建 **PROMPT_CONFIGS** 参数化系统
- 添加 `get_prompt_config()` 函数
- 修改 `main()` 支持：
  - 正样本仅
  - 正样本 + 负样本
  - 负样本仅
- 更新 `rewrite_file_with_validation()` 函数签名接收 `fixing_prompt`
- 使用参数化的 prompt 而非全局常量

### 3. evaluation/compare_all.py 改动 ✓
- 同时加载 examples/ 和 examples_negative/
- 调整编译环境处理以支持两种示例
- 添加样本类型标记到输出
- 在 JSON 结果中记录 is_negative 标记

---

## 📐 架构设计

### PROMPT_CONFIGS 参数化系统

```
SYSTEM_PROMPTS (全局常量字典)
        ↓
PROMPT_CONFIGS (参数化组合)
        ├─ prompt_idx: 0 → {system_prompt: SYSTEM_PROMPT_0, fixing_prompt: FIXING_PROMPT}
        ├─ prompt_idx: 1 → {system_prompt: SYSTEM_PROMPT_1, fixing_prompt: FIXING_PROMPT}
        ├─ prompt_idx: 3 → {system_prompt: SYSTEM_PROMPT_3, fixing_prompt: CUSTOM_FIXING_3}
        └─ ... (可扩展)

main() 使用流程：
    1. 解析 --include-negative / --negative-only
    2. 调用 get_prompt_config(prompt_idx)
    3. 获取 system_prompt, fixing_prompt
    4. 选择数据源 (positive / negative / both)
    5. 迭代处理示例
```

### 负样本处理流程

```
run.sh --include-negative
    ↓
refractor.py 判断
    ├─ positive only: /home/guoxy/concrat/examples/*/main.c2rust.rs
    ├─ positive + negative: examples/ + examples_negative/
    └─ negative only: /home/guoxy/concrat/examples_negative/*/main.c2rust.rs
    ↓
标记示例类型 ([POS] 或 [NEG])
    ↓
输出到 result/TIMESTAMP_prompt_idx_strategy/examples/{name}/final.rs
    ↓
evaluation/compare_all.py
    ├─ 加载结果
    ├─ 根据路径判断 is_negative
    ├─ 使用正确的 example_dir 进行编译
    └─ 输出带标记的报告
```

---

## 🧪 测试验证

### 语法检查 ✓
```bash
✅ refractor.py Syntax OK
✅ compare_all.py Syntax OK
✅ run.sh Syntax OK
```

### 使用示例

#### 1. 运行仅正样本（传统方式）
```bash
./run.sh 6 --validate --strategy compile
```
输出：
```
🔎 Found 33 examples to process (positive only)

[POS] 🔄 array_const
[POS] 🔄 array_main
...
```

#### 2. 运行正样本 + 负样本
```bash
./run.sh 6 --validate --strategy compile --include-negative
```
输出：
```
📊 Examples: positive + negative (33 + 5)
🔎 Found 38 examples to process (positive + negative)

[POS] 🔄 array_const
[POS] 🔄 array_main
...
[NEG] 🔄 array_const____deadlock
[NEG] 🔄 array_main____partial_critical_section
...
```

#### 3. 运行仅负样本
```bash
./run.sh 6 --validate --strategy compile --negative-only
```
输出：
```
🔎 Found 5 examples to process (negative only)

[NEG] 🔄 array_const____deadlock
[NEG] 🔄 array_main____lock_leak
...
```

---

## 📊 输出示例

### comparison_report.json 中的变化

**before**:
```json
{
  "name": "array_simple",
  "original": {...},
  "concrat": {...},
  "llm": {...}
}
```

**after**:
```json
{
  "name": "array_const____deadlock",
  "is_negative": true,  // ← 新增
  "original": {...},
  "concrat": {...},
  "llm": {...}
}
```

### 评估输出中的标记

**before**:
```
array_const            │ original  │ (base)   │ 7 │ ...
                       │ concrat   │ ✅       │ ...
                       │ LLM       │ ✅       │ ...
```

**after**:
```
[POS] array_const      │ original  │ (base)   │ 7 │ ...
                       │ concrat   │ ✅       │ ...
                       │ LLM       │ ✅       │ ...

[NEG] array_const____deadlock │ original │ (base) │ ...
                              │ concrat  │ ✅     │ ...
                              │ LLM      │ ❌     │ ...
```

---

## 🔧 关键改动详情

### refractor.py - 参数化系统

```python
# 新增 PROMPT_CONFIGS（第 208-237 行）
PROMPT_CONFIGS = {...}

def get_prompt_config(prompt_idx: int) -> dict:
    """获取 prompt 配置"""
    if prompt_idx not in PROMPT_CONFIGS:
        raise ValueError(...)
    return PROMPT_CONFIGS[prompt_idx]
```

**优势**:
- 支持每个 prompt_idx 配置不同的 fixing_prompt
- 易于添加新 prompt 组合
- 代码结构清晰，便于维护

### refractor.py - main() 函数

```python
# 参数解析（第 763-764 行）
include_negative = "--include-negative" in sys.argv
negative_only = "--negative-only" in sys.argv

# 示例源选择（第 777-789 行）
if negative_only:
    sample_type = "negative only"
    examples = sorted(glob.glob("/home/guoxy/concrat/examples_negative/*/main.c2rust.rs"))
elif include_negative:
    sample_type = "positive + negative"
    positive = sorted(...)
    negative = sorted(...)
    examples = positive + negative
else:
    sample_type = "positive only"
    examples = sorted(glob.glob("/home/guoxy/concrat/examples/*/main.c2rust.rs"))

# 标记示例（第 823-825 行）
is_negative = "/examples_negative/" in filepath
sample_prefix = "[NEG]" if is_negative else "[POS]"
```

### evaluate/compare_all.py - 双示例支持

```python
# 加载正负示例（第 228-237 行）
positive_examples = sorted([...glob.glob(f"{EXAMPLES_DIR}/*/")...])
negative_examples = sorted([...glob.glob(f".../examples_negative/*/")...])
examples = positive_examples + negative_examples

# 编译环境处理（第 254-260 行）
is_negative = "examples_negative" in example_dir
if llm_output_dir:
    if is_negative:
        example_dir_for_compile = os.path.join(".../examples_negative", name)
    else:
        example_dir_for_compile = os.path.join(EXAMPLES_DIR, name)
```

---

## 📚 文档

已创建的文档：
1. **NEGATIVE_SAMPLES_AND_PROMPT_CONFIG.md** - 完整实现文档
2. **QUICK_START_NEGATIVE_SAMPLES.md** - 快速使用指南
3. **COMPILE_MISMATCH_ANALYSIS.md** - 编译不一致问题分析

---

## 🚀 后续扩展可能性

### 1. 多 Prompt 集合
```python
PROMPT_CONFIGS_SET_A = {...}  # 激进转换
PROMPT_CONFIGS_SET_B = {...}  # 保守转换

# 在 main() 中选择
prompt_set = "--prompt-set" 和 sys.argv 中解析
```

### 2. 条件化 Prompt
```python
# 根据示例特性选择 prompt
def select_prompt(example_name: str, complexity: str):
    if "deadlock" in example_name:
        return PROMPT_CONFIGS[7]  # 特殊 prompt
    elif complexity == "high":
        return PROMPT_CONFIGS[6]  # 详细 prompt
    else:
        return PROMPT_CONFIGS[3]  # 标准 prompt
```

### 3. 独立报告
```bash
# 为正负样本分别生成报告
python3 evaluation/generate_split_reports.py
# → reports/
#   ├── positive_only/ (comparison_report.json)
#   ├── negative_only/ (comparison_report.json)
#   └── differences.md
```

---

## ✨ 关键特性

| 特性 | 说明 | 文件 |
|------|------|------|
| 参数化 Prompt | PROMPT_CONFIGS 字典系统，易于扩展 | refractor.py |
| 负样本支持 | --include-negative, --negative-only | run.sh |
| 双示例源 | 同时处理 examples/ 和 examples_negative/ | refractor.py, compare_all.py |
| 编译环境修复 | 正确的 example_dir 用于编译 | compare_all.py |
| 样本标记 | [POS]/[NEG] 标记和 is_negative 字段 | 所有脚本 |
| 灵活的 fixing_prompt | 支持不同 prompt_idx 使用不同 fixing_prompt | refractor.py |

---

## 📋 文件修改清单

| 文件 | 修改行数 | 改动类型 |
|------|---------|---------|
| run.sh | +10 | 参数解析、帮助信息、传参 |
| refractor.py | +60 | PROMPT_CONFIGS、main() 扩展、函数签名 |
| evaluation/compare_all.py | +30 | 负样本加载、编译环境、标记输出 |
| **新增文档** | **3** | 实现文档、快速指南、分析报告 |

---

## 🎯 后续任务

### 完成的任务 ✅
- [x] 添加 --include-negative 和 --negative-only 参数
- [x] 参数化 Prompt 系统
- [x] 支持负样本在整个流程中
- [x] 编译环境修复
- [x] 输出标记和文档

### 可选任务 🔮
- [ ] 为不同 prompt_idx 配置不同的 fixing_prompt
- [ ] 生成独立的正/负样本报告
- [ ] 添加负样本成功率聚合统计
- [ ] 创建 prompt 版本管理系统

---

## 💡 设计思想

### 为什么这样设计？

1. **PROMPT_CONFIGS 字典**
   - 避免硬编码全局变量
   - 支持运行时动态配置
   - 便于版本管理和 A/B 测试

2. **参数化 fixing_prompt**
   - 不同 system_prompt 可能需要不同的修复策略
   - 使用参数而非全局常量
   - 易于测试不同 prompt 组合

3. **统一的负样本处理**
   - 不区分正负样本的代码接口
   - 通过 is_negative 标记识别类型
   - 自动选择正确的编译环境

4. **输出标记**
   - 便于后续分析和统计
   - 支持生成独立报告
   - 提高可视化和可读性

---

## 🔗 相关资源

- 🔍 [编译环境差异分析](COMPILE_MISMATCH_ANALYSIS.md)
- 📖 [负样本完整文档](NEGATIVE_SAMPLES_AND_PROMPT_CONFIG.md)
- 🚀 [快速开始指南](QUICK_START_NEGATIVE_SAMPLES.md)
- 📝 [项目 README](README.md)
