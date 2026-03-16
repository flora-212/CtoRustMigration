# run.sh 使用指南

## 概述

`run.sh` 是一个完整的流程自动化脚本，整合了：
1. **Ollama LLM 启动** - 自动启动本地 LLM
2. **代码生成** - 调用 refractor.py 生成或改进代码
3. **迭代验证** - （新增）基于验证器的迭代反馈循环
4. **评估套件** - 运行评估脚本
5. **对比报告** - 生成对比分析报告

## 快速开始

### 默认使用（推荐）
```bash
cd /home/guoxy/concrat/LLM
./run.sh
```

**默认配置**:
- 使用 SYSTEM_PROMPT_1
- 启用验证循环
- 编译检查策略
- 最多 3 次迭代

### 显式启用验证
```bash
./run.sh 1 --validate
```

### 使用全面验证
```bash
./run.sh 1 --validate --strategy comprehensive
```

### 增加迭代次数（更好的结果，更慢）
```bash
./run.sh 1 --validate --max-iterations 5
```

## 命令参数详解

### 位置参数

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `prompt_idx` | 系统提示词版本 (0/1/2/...) | 1 |

```bash
./run.sh 0                   # 使用 SYSTEM_PROMPT_0
./run.sh 1                   # 使用 SYSTEM_PROMPT_1
./run.sh 2                   # 使用 SYSTEM_PROMPT_2
```

### 验证选项

| 选项 | 说明 | 默认值 |
|------|------|--------|
| `--validate` | 启用迭代验证循环 | 已启用 |
| `--no-validate` | 禁用验证循环 | - |
| `--strategy STRATEGY` | 验证策略 | compile |
| `--max-iterations N` | 最大迭代次数 | 3 |

#### 验证策略说明

| 策略 | 速度 | 检查内容 | 用途 |
|------|------|--------|------|
| `compile` | ⚡⚡⚡ | 代码是否编译成功 | 快速迭代 |
| `safety` | ⚡⚡ | unsafe、pthread、raw_ptr | 安全评估 |
| `lock_safety` | ⚡⚡ | 锁模式、thread join | 并发分析 |
| `comprehensive` | ⚡ | 全部检查 | 最终验收 |

```bash
# 编译验证（最快）
./run.sh 1 --strategy compile

# 安全检查
./run.sh 1 --strategy safety

# 锁安全分析
./run.sh 1 --strategy lock_safety

# 全面检查（综合体）
./run.sh 1 --strategy comprehensive
```

### 强制选项

| 选项 | 说明 |
|------|------|
| `--force` | 强制所有步骤重新执行 |
| `--force-rewrite` | 仅强制代码生成步骤 |
| `--force-eval` | 仅强制评估步骤 |
| `--force-generate` | 仅强制对比报告生成 |
| `--clear` | 清空缓存并强制所有步骤 |

```bash
# 强制全部重新执行
./run.sh 1 --force

# 仅重新生成代码
./run.sh 1 --force-rewrite

# 强制重新评估
./run.sh 1 --force-eval
```

### 其他选项

| 选项 | 说明 |
|------|------|
| `--verbose` | 详细输出（显示所有中间步骤） |
| `--help` | 显示帮助信息 |

## 使用示例

### 场景 1: 快速测试（编译验证）
```bash
./run.sh 1 --validate --strategy compile
```

**特点**:
- 最快的验证方式
- 检查代码是否能编译
- 约 2-3 分钟完成

### 场景 2: 安全性优化
```bash
./run.sh 1 --validate --strategy safety
```

**特点**:
- 检查 unsafe 块、pthread 调用等
- 生成的代码会更安全
- 约 3-4 分钟完成

### 场景 3: 并发代码优化
```bash
./run.sh 1 --validate --strategy lock_safety
```

**特点**:
- 分析锁的使用模式
- 检查线程 join() 等并发特性
- 最适合含有 pthread/mutex 的代码

### 场景 4: 最严格验证
```bash
./run.sh 1 --validate --strategy comprehensive --max-iterations 5
```

**特点**:
- 最全面的检查
- 更多迭代次数以获得最好结果
- 约 5-8 分钟完成

### 场景 5: 比较不同版本
```bash
# 生成版本 0
./run.sh 0 --force

# 生成版本 1
./run.sh 1 --force

# 生成版本 2
./run.sh 2 --force

# 对比评估
python3 compare_all.py 1
```

### 场景 6: 强制更新所有版本
```bash
./run.sh 1 --force --validate --strategy comprehensive
```

**执行**:
- 删除旧的生成文件
- 重新生成代码
- 运行完整的验证和评估

## 流程详解

运行 `./run.sh` 时会执行以下步骤：

### Step 1: 启动 Ollama LLM
```
✅ Ollama 已在运行
或
🚀 启动 Ollama...
   Ollama PID: 1234
   等待 Ollama 就绪...
✅ Ollama 已就绪
```

### Step 2: 生成代码（+ 迭代验证）
```
🔄 Step 2: 生成代码 (SYSTEM_PROMPT_1)
   + 迭代验证循环 (策略: compile, 最大迭代: 3)

[1/30] Processing: array_const
  -> 使用迭代验证 (最多 3 次迭代)...
    [Iteration 1/3] ❌ 验证失败
      📋 验证反馈详情:
      ❌ [compile] Compilation failed:
         └─ error[E0412]: cannot find type `T` in this scope
      🔄 向 LLM 反馈错误信息...

    [Iteration 2/3] ❌ 验证失败
      📋 验证反馈详情:
      ❌ [safety] Safety concerns: 2 unsafe blocks
      🔄 向 LLM 反馈错误信息...

    [Iteration 3/3] ✅ 验证通过！
      ✅ [compile] Code compiles successfully
      ✅ [safety] No major safety concerns
  
  -> ✅ 保存到: examples/array_const/main_rewritten_1.rs

[2/30] Processing: array_main
...
```

### Step 3: 运行评估套件
```
📊 Step 3: 运行评估套件
  - 编译所有版本
  - 检查安全指标
  - 生成中间报告
```

### Step 4: 生成对比报告
```
📈 Step 4: 生成对比报告
  - 对比原始 vs 生成的代码
  - 分析改进指标
  - 输出最终报告
```

### 完成
```
╔════════════════════════════════════════════════════════════════════════════╗
║                       ✅ 流程执行完成！                                    ║
╚════════════════════════════════════════════════════════════════════════════╝

📁 输出文件位置:
   - 生成的代码: examples/*/main_rewritten_1.rs
   - 对比报告: result/1/comparison_report.json
   - 评估结果: result/1/

🔍 查看结果:
   cat result/1/comparison_report.json | jq .
```

## 输出解释

### 验证器输出详解

**编译成功**:
```
[Iteration 1/3] ✅ 验证通过！
  ✅ [compile] Code compiles successfully
```

**编译失败**:
```
[Iteration 1/3] ❌ 验证失败
  📋 验证反馈详情:
  ❌ [compile] Compilation failed:
     └─ error[E0412]: cannot find type `Foo` in this scope
  🔄 向 LLM 反馈错误信息...
```

**安全问题**:
```
[Iteration 2/3] ❌ 验证失败
  📋 验证反馈详情:
  ❌ [safety] Safety concerns: 3 unsafe block(s), 2 pthread call(s)
  🔄 向 LLM 反馈错误信息...
```

**全部通过**:
```
[Iteration 3/3] ✅ 验证通过！
  ✅ [compile] Code compiles successfully
  ✅ [safety] No major safety concerns
  ✅ [lock_safety] Lock safety OK (uses std::sync::Mutex)
```

## 常见问题

### Q1: 如何看到每一轮验证的错误信息？
A: 默认会显示详细的错误信息。如需更详细输出，使用 `--verbose`：
```bash
./run.sh 1 --verbose
```

### Q2: 为什么有些代码迭代多次还是失败？
A: 可能原因：
1. 代码结构太复杂
2. 验证策略过于严格
3. LLM 模型能力限制

**解决**:
- 减少迭代次数（快速失败）
- 使用更快的策略（compile 而非 comprehensive）
- 尝试其他 prompt 版本

### Q3: 可以同时运行多个版本吗？
A: 不推荐，因为会竞争计算资源。建议按顺序运行：
```bash
./run.sh 0 --force  # 版本 0
./run.sh 1 --force  # 版本 1
./run.sh 2 --force  # 版本 2
```

### Q4: 如何清理旧的生成文件？
A: 使用 cleanup.py：
```bash
python3 cleanup.py --all --dry-run    # 预览
python3 cleanup.py --keep 1 --yes     # 只保留版本 1
```

### Q5: 修改了 system prompt，如何看到效果？
A: 使用 `--force` 强制重新生成：
```bash
./run.sh 1 --force --validate
```

## 配置文件说明

run.sh 中可调整的参数：

```bash
# 在 run.sh 中修改这些值
PROMPT_IDX=1                    # 默认 prompt 索引
VALIDATE_FLAG="--validate"      # 是否启用验证
STRATEGY="compile"              # 默认验证策略
MAX_ITERATIONS=3                # 默认最大迭代次数
```

## 高级用法

### 仅生成代码，不评估
```bash
./run.sh 1 --validate --force-rewrite
```

### 跳过生成，只评估
```bash
./run.sh 1 --force-eval
```

### 生成中间结果日志
```bash
./run.sh 1 --validate --verbose 2>&1 | tee run_$(date +%s).log
```

### 集成到 CI/CD
```bash
#!/bin/bash
cd /home/guoxy/concrat/LLM

# 生成所有版本
for idx in 0 1 2; do
    ./run.sh $idx --force --validate --strategy compile
    if [ $? -ne 0 ]; then
        echo "❌ 版本 $idx 失败"
        exit 1
    fi
done

echo "✅ 所有版本生成成功"
```

## 故障排除

### 问题: Ollama 连接失败
```
❌ Ollama 启动超时
```

**解决**:
1. 检查 Ollama 是否安装和响应
2. 手动启动: `ollama serve`
3. 验证端口: `curl localhost:11434/api/tags`

### 问题: LLM 调用超时
```
LLM 响应超时或无响应
```

**解决**:
1. 查看 Ollama 日志
2. 减小 max_iterations
3. 使用更快的策略

### 问题: 编译失败持续重复
```
所有迭代都显示编译错误
```

**解决**:
1. 检查原始代码是否有极端复杂性
2. 尝试不同的 prompt 版本
3. 手动检查错误信息

## 监控和调试

### 查看 Ollama 状态
```bash
ps aux | grep ollama
```

### 查看生成的代码
```bash
cat examples/struct_init/main_rewritten_1.rs | head -50
```

### 对比两个版本
```bash
diff -u examples/struct_init/main_rewritten_0.rs examples/struct_init/main_rewritten_1.rs
```

### 查看对比报告
```bash
cat result/1/comparison_report.json | jq '.[] | select(.name=="struct_init")'
```

---

**版本**: 2.0  
**更新**: 2024  
**状态**: ✅ 完整可用
