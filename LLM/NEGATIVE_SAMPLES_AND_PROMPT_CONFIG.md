# 负样本支持与参数化 Prompt 系统 - 实现总结

## 概述
添加了对负样本（examples_negative）的完整支持，并创建了参数化的 Prompt 管理系统，便于后续扩展不同的 system_prompt 和 fixing_prompt 组合。

---

## 一、run.sh 改动

### 添加的新参数

```bash
--include-negative      # 同时处理正样本和负样本
--negative-only         # 仅处理负样本
```

### 参数处理逻辑

在参数解析 while 循环中添加：
```bash
--include-negative)
    INCLUDE_NEGATIVE="--include-negative"
    ;;
--negative-only)
    NEGATIVE_ONLY="--negative-only"
    INCLUDE_NEGATIVE="--include-negative"
    ;;
```

### 传递给 refractor.py

```bash
python3 "$SCRIPT_DIR/refractor.py" "$PROMPT_IDX" \
    ... 
    $INCLUDE_NEGATIVE \
    $NEGATIVE_ONLY \
    ...
```

### 配置输出

在输出中显示当前样本类型：
```bash
echo "   - Include Negative: ${INCLUDE_NEGATIVE:-(no)}"
if [ ! -z "$NEGATIVE_ONLY" ]; then
    echo "   - Negative Only: yes"
fi
```

---

## 二、refractor.py 核心改动

### 1. 参数化 Prompt 系统

**创建 PROMPT_CONFIGS 字典**：
```python
PROMPT_CONFIGS = {
    # prompt_idx: {
    #     "system_prompt": SYSTEM_PROMPT_X,
    #     "fixing_prompt": FIXING_PROMPT
    # }
}

# 初始化所有可用的 prompt 组合
for idx in SYSTEM_PROMPTS.keys():
    PROMPT_CONFIGS[idx] = {
        "system_prompt": SYSTEM_PROMPTS[idx],
        "fixing_prompt": FIXING_PROMPT,  # 当前所有使用同一个，便于后续定制
    }

def get_prompt_config(prompt_idx: int) -> dict:
    """获取特定 prompt_idx 的配置"""
    if prompt_idx not in PROMPT_CONFIGS:
        raise ValueError(f"Prompt config {prompt_idx} not found")
    return PROMPT_CONFIGS[prompt_idx]
```

**优势**：
- 便于后续添加新的 prompt 组合
- 支持针对不同 prompt_idx 使用不同的 fixing_prompt
- 代码易于扩展和维护

### 2. main() 函数改动

#### 参数解析
```python
include_negative = "--include-negative" in sys.argv
negative_only = "--negative-only" in sys.argv
```

#### 获取 Prompt 配置
```python
prompt_config = get_prompt_config(prompt_idx)
system_prompt = prompt_config["system_prompt"]
fixing_prompt = prompt_config["fixing_prompt"]
```

#### 示例源选择逻辑
```python
if negative_only:
    sample_type = "negative only"
    examples = sorted(glob.glob("/home/guoxy/concrat/examples_negative/*/main.c2rust.rs"))
elif include_negative:
    sample_type = "positive + negative"
    positive = sorted(glob.glob("/home/guoxy/concrat/examples/*/main.c2rust.rs"))
    negative = sorted(glob.glob("/home/guoxy/concrat/examples_negative/*/main.c2rust.rs"))
    examples = positive + negative
else:
    sample_type = "positive only"
    examples = sorted(glob.glob("/home/guoxy/concrat/examples/*/main.c2rust.rs"))
```

#### 输出中标记样本类型
```python
for i, filepath in enumerate(examples, 1):
    ...
    is_negative = "/examples_negative/" in filepath
    sample_prefix = "[NEG]" if is_negative else "[POS]"
    print(f"[{i:2d}/{total}] {sample_prefix} 🔄 {example_name}")
```

### 3. 函数签名更新

**rewrite_file_with_validation**：
```python
def rewrite_file_with_validation(
    filepath: str,
    system_prompt: str,
    fixing_prompt: str,  # ← 新增参数
    example_dir: str,
    max_iterations: int = 5,
    validation_strategy: ValidationStrategy = ValidationStrategy.COMPILE,
    output_manager: OutputManager = None,
    example_name: str = None
) -> tuple:
    ...
    # 函数体内使用参数传入的 fixing_prompt
    messages.append({
        "role": "user", 
        "content": fixing_prompt.format(feedback=llm_feedback)  # ← 使用参数而非全局常量
    })
```

### 4. main() 调用处更新

```python
passed, code, report, iterations_used = rewrite_file_with_validation(
    filepath,
    system_prompt,
    fixing_prompt,  # ← 传入参数化的 fixing_prompt
    example_dir,
    max_iterations=max_iterations,
    validation_strategy=validation_strategy,
    output_manager=output_manager,
    example_name=example_name
)
```

---

## 三、evaluation/compare_all.py 改动

### 1. 示例加载逻辑

```python
# 加载正样本和负样本
positive_examples = sorted([d for d in glob.glob(f"{EXAMPLES_DIR}/*/") if os.path.isdir(d)])
negative_examples = sorted([d for d in glob.glob(f"/home/guoxy/concrat/examples_negative/*/") if os.path.isdir(d)])
examples = positive_examples + negative_examples

sample_type = "positive + negative" if negative_examples else "positive only"
print(f"📊 Examples: {sample_type} ({len(positive_examples)} + {len(negative_examples)})")

# 仅为正样本构建 concrat 缓存（負样本无 concrat 版本）
concrat_cache = load_concrat_cache()
if concrat_cache is None or clear:
    concrat_cache = build_concrat_cache(positive_examples)  # 仅正样本
```

### 2. 编译环境处理

```python
for example_dir in examples:
    ...
    is_negative = "examples_negative" in example_dir
    
    if llm_output_dir:
        llm_rs = os.path.join(llm_output_dir, "examples", name, "final.rs")
        # 使用正确的源目录进行编译上下文
        if is_negative:
            example_dir_for_compile = os.path.join("/home/guoxy/concrat/examples_negative", name)
        else:
            example_dir_for_compile = os.path.join(EXAMPLES_DIR, name)
```

### 3. 输出标记

```python
row = {"name": name}
row["original"] = {"metrics": orig_metrics, "lock_safety": orig_lock}
row["is_negative"] = is_negative  # ← 记录样本类型

# 打印时添加样本标记
sample_mark = "[NEG]" if is_negative else "[POS]"
print(f"  {sample_mark} {name:<16} │ {'original':<10} │ ...")
```

---

## 四、工作流程

### 标准使用方式

#### 仅运行正样本（默认）
```bash
./run.sh 3 --validate --strategy compile
```

#### 运行正样本 + 负样本
```bash
./run.sh 3 --validate --strategy compile --include-negative
```

#### 仅运行负样本
```bash
./run.sh 3 --validate --strategy compile --negative-only
```

### 完整执行流程

1. **refractor.py** (Step 2)
   - 读取 examples/ 或 examples_negative/ 或两者
   - 根据 PROMPT_CONFIGS 获取 system_prompt 和 fixing_prompt
   - 生成/迭代改进代码
   - 保存到时间戳目录 `result/TIMESTAMP_{prompt_idx}_{strategy}/`

2. **run_evaluation.sh** (Step 3)
   - 调用编译和评估脚本
   - 执行 concrat 缓存和评估

3. **compare_all.py** (Step 4)
   - 读取时间戳目录中的 final.rs
   - 对正样本和负样本分别评估
   - 在输出中标记 [POS] 或 [NEG]
   - 生成 comparison_report.json

---

## 五、扩展性设计

### 后续添加新 Prompt 组合的方法

#### 示例：为 prompt_idx=7 创建特殊的 fixing_prompt

```python
# 1. 定义新的 FIXING_PROMPT 变量
FIXING_PROMPT_7 = """Your custom fixing prompt for prompt_idx 7..."""

# 2. 在 PROMPT_CONFIGS 初始化后覆盖
PROMPT_CONFIGS[7] = {
    "system_prompt": SYSTEM_PROMPTS[7],
    "fixing_prompt": FIXING_PROMPT_7,
}
```

#### 示例：为负样本使用特殊的 system_prompt

```python
# 在 refractor.py 的 main() 中
if is_negative:
    # 负样本使用更激进的 prompt
    system_prompt = SYSTEM_PROMPTS.get(prompt_idx + 10, system_prompt)
```

---

## 六、文件更改清单

### 已修改的文件

1. **run.sh**
   - 添加 INCLUDE_NEGATIVE 和 NEGATIVE_ONLY 变量
   - 添加参数解析逻辑
   - 传递新参数给 refractor.py

2. **refractor.py**
   - 创建 PROMPT_CONFIGS 字典系统
   - 添加 get_prompt_config() 函数
   - 修改 main() 支持负样本
   - 更新 rewrite_file_with_validation() 函数签名

3. **evaluation/compare_all.py**
   - 加载正负样本
   - 调整编译环境处理
   - 添加样本类型标记到输出

---

## 七、测试清单

- [ ] 运行仅正样本：`./run.sh 6 --validate --strategy compile`
- [ ] 运行正+负样本：`./run.sh 6 --validate --strategy compile --include-negative`
- [ ] 运行仅负样本：`./run.sh 6 --validate --strategy compile --negative-only`
- [ ] 验证输出中的 [POS]/[NEG] 标记
- [ ] 验证 comparison_report.json 中对负样本的正确处理
- [ ] 验证不同 prompt_idx 的 prompt 配置加载正确

---

## 八、未来改进方向

1. **多 Prompt 集合**
   - 为不同的转换目标创建 PROMPT_SET_A, PROMPT_SET_B 等
   - 支持在 run.sh 中选择 prompt 集合

2. **条件化 Prompt**
   - 根据示例的复杂度或特性动态选择 prompt
   - 例如：复杂的多线程示例使用更详细的 prompt

3. **负样本验证**
   - 添加专门的负样本验证策略（预期失败场景）
   - 验证"错误代码被正确拒绝"的能力

4. **性能指标**
   - 为正负样本分别统计成功率
   - 生成独立的报告来分析各类示例的转换难度
