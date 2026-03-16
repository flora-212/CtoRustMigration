# LLM 代码转换系统 - 使用指南

## 📋 目录
1. [快速开始](#快速开始)
2. [输出结构](#输出结构)
3. [详细使用](#详细使用)
4. [常用命令](#常用命令)
5. [常见问题](#常见问题)
6. [技术细节](#技术细节)

---

## 快速开始

### 基本命令

```bash
cd /home/guoxy/concrat/LLM

# 一键启动完整流程（自动创建时间戳目录）
./run.sh 0 --validate --strategy compile
```

这会执行：
1. 生成代码
2. 进行迭代验证
3. 运行评估
4. 生成报告
5. 所有结果保存到 `result/{YYYYMMDD_HHMMSS}_{PROMPT_IDX}/`

### 查看结果

```bash
# 获取最新结果目录
LATEST=$(ls result/ | sort -r | head -1)
echo $LATEST

# 查看配置
cat result/$LATEST/config.json | jq .

# 查看某个示例的输出
cat result/$LATEST/examples/array_const/main_rewritten.rs

# 查看评估报告
cat result/$LATEST/evaluation/comparison_report.json | jq .
```

---

## 输出结构

所有输出按**时间戳**组织，每次运行创建一个独立目录。

### 完整目录树

```
result/
└── 20260316_162332_0/          # 时间戳命名 (YYYYMMDD_HHMMSS_PROMPTIDX)
    ├── config.json              # ✓ 运行配置和统计信息
    │
    ├── rewritten/               # ✓ 所有代码重写文件
    │   ├── round1/              # 第1轮迭代结果
    │   │   └── main_rewritten.rs
    │   ├── round2/              # 第2轮迭代结果（如需修改）
    │   │   └── main_rewritten.rs
    │   └── final/               # ✓ 最终通过的结果
    │       └── main_rewritten.rs
    │
    ├── examples/                # ✓ 30个示例各有一个子文件夹
    │   ├── array_const/
    │   │   └── main_rewritten.rs
    │   ├── array_main/
    │   │   └── main_rewritten.rs
    │   └── ... (28个其他示例)
    │
    └── evaluation/              # ✓ 评估和测试结果
        ├── comparison_report.json
        ├── comparison_report.md
        ├── clippy_concurrency_report.json
        └── clippy_concurrency_report.md
```

### 配置文件 (config.json)

示例内容：

```json
{
  "timestamp": "20260316_162332",
  "prompt_idx": 0,
  "validate": true,
  "strategy": "compile",
  "max_iterations": 3,
  "force": false,
  "start_time": "2026-03-16T16:23:32...",
  "end_time": "2026-03-16T16:45:12...",
  "total_files": 30,
  "success_count": 28,
  "failed_count": 2,
  "failed_examples": ["example_005", "example_012"]
}
```

---

## 详细使用

### 1. 基本参数说明

```bash
./run.sh [PROMPT_IDX] [OPTIONS]

参数:
  PROMPT_IDX           0, 1, 2 (使用不同的系统提示词)
  
选项:
  --validate           启用迭代验证（默认）
  --no-validate        禁用验证，直接生成
  --strategy STRATEGY  验证策略 (compile, safety, lock_safety, comprehensive)
  --max-iterations N   最大迭代次数（默认: 3）
  --force              强制重新生成所有文件
  --verbose            详细输出
```

### 2. 常见场景

#### 场景 A: 快速生成（无验证）

```bash
./run.sh 0 --no-validate
```

优点: 快速
缺点: 代码质量无保证

#### 场景 B: 标准验证（推荐）

```bash
./run.sh 0 --validate --strategy compile
```

优点: 平衡速度和质量
缺点: 需要 30-60 分钟

#### 场景 C: 严格验证

```bash
./run.sh 0 --validate --strategy comprehensive
```

优点: 最高质量
缺点: 需要 1-2 小时

#### 场景 D: 增加重试次数

```bash
./run.sh 0 --validate --max-iterations 5
```

优点: 更多尝试机会
缺点: 更耗时

#### 场景 E: 强制重新生成

```bash
./run.sh 0 --force
```

会重新生成所有文件，即使已存在。

### 3. 迭代过程

启用验证时的迭代过程：

```
Round 1: 生成代码
         ↓ 验证
         ✗ 失败 → LLM 反馈错误
         
Round 2: 改进代码
         ↓ 验证
         ✗ 失败 → LLM 反馈错误
         
Round 3: 继续改进
         ↓ 验证
         ✓ 成功 → 保存为 final/
```

每一轮的结果都被保存到 `round{N}/` 目录。

### 4. 查看迭代过程

```bash
LATEST=$(ls result/ | sort -r | head -1)

# 对比各轮次的代码变化
diff result/$LATEST/rewritten/round1/main_rewritten.rs \
     result/$LATEST/rewritten/final/main_rewritten.rs

# 查看第1轮的代码
head -50 result/$LATEST/rewritten/round1/main_rewritten.rs

# 查看最终的代码
head -50 result/$LATEST/rewritten/final/main_rewritten.rs
```

---

## 常用命令

### 查看结果

```bash
# 列出所有已生成的结果
ls -1 result/

# 查看最新结果
LATEST=$(ls result/ | sort -r | head -1)
echo "最新结果: $LATEST"

# 查看结果目录结构
tree result/$LATEST/
```

### 统计信息

```bash
# 成功/失败数
jq '.success_count, .failed_count' result/$LATEST/config.json

# 失败的示例
jq '.failed_examples' result/$LATEST/config.json

# 所有配置信息
jq . result/$LATEST/config.json
```

### 审查代码

```bash
# 查看某个示例的最终结果
cat result/$LATEST/examples/array_const/main_rewritten.rs | head -50

# 查看所有示例的行数统计
for dir in result/$LATEST/examples/*/; do
  lines=$(wc -l < "$dir/main_rewritten.rs")
  echo "$(basename "$dir"): $lines lines"
done
```

### 对比分析

```bash
# 对比两次运行的结果
LATEST=$(ls result/ | sort -r | head -1)
PREVIOUS=$(ls result/ | sort -r | head -2 | tail -1)

echo "=== 配置对比: $PREVIOUS vs $LATEST ==="
diff <(jq . result/$PREVIOUS/config.json) \
     <(jq . result/$LATEST/config.json)

echo "=== 成功率对比 ==="
echo "Previous: $(jq '.success_count' result/$PREVIOUS/config.json)/$(jq '.total_files' result/$PREVIOUS/config.json)"
echo "Latest:   $(jq '.success_count' result/$LATEST/config.json)/$(jq '.total_files' result/$LATEST/config.json)"
```

### 数据导出

```bash
# 导出为 CSV 用于 Excel 分析
jq -r '.[] | [.strategy, .success_count, .total_files] | @csv' \
  result/$LATEST/config.json > analysis.csv

# 导出评估报告
cp result/$LATEST/evaluation/comparison_report.json report.json
```

### 清理操作

```bash
# 清理主输出文件（保留时间戳目录）
python3 cleanup.py --all --yes

# 删除特定的时间戳目录
rm -rf result/20260316_162332_0

# 删除较旧的时间戳目录（保留最新 5 个）
ls -t result/ | tail -n +6 | xargs -I {} rm -rf result/{}
```

### 运行测试

```bash
# 完整系统检查
bash test_output_structure.sh

# 验证单个文件
python3 validator.py examples/array_const/main_rewritten_0.rs
```

---

## 常见问题

### Q: 新的系统会影响性能吗？

**A:** 不会。目录操作开销可忽略不计 (< 1%)。

### Q: 旧的脚本还能用吗？

**A:** 是的。向后兼容性已保留：
- `examples/*/main_rewritten_*.rs` 继续被更新
- `result/{PROMPT_IDX}/` 继续保留最新评估结果

### Q: 时间戳目录中可以有 deps_rewritten 吗？

**A:** 可以。目录结构已预留该位置。

### Q: 如何只保存最后的结果，不保存中间轮次？

**A:** 系统会自动保存。但中间结果被保留以供分析。如需删除中间结果：

```bash
LATEST=$(ls result/ | sort -r | head -1)
rm -rf result/$LATEST/rewritten/round*  # 仅保留 final/
```

### Q: 如何快速查看某个示例的改进？

**A:** 

```bash
LATEST=$(ls result/ | sort -r | head -1)
EXAMPLE=array_const

echo "=== 第1轮 ==="
head -20 result/$LATEST/rewritten/round1/main_rewritten.rs

echo "=== 最终版 ==="
head -20 result/$LATEST/rewritten/final/main_rewritten.rs

echo "=== 原始文件 ==="
head -20 examples/$EXAMPLE/main.rs
```

### Q: 如何自动比较多次运行？

**A:** 使用提供的脚本或创建 Python 分析脚本：

```python
import json
import os
from pathlib import Path

result_dir = Path("result")
runs = sorted(result_dir.glob("*_0"), reverse=True)

for run in runs[:3]:
    config = json.load(open(run / "config.json"))
    print(f"{run.name}: {config['success_count']}/{config['total_files']}")
```

---

## 技术细节

### 核心模块

#### output_manager.py

管理时间戳目录和输出文件：

```python
from output_manager import OutputManager

# 初始化
mgr = OutputManager()
output_root = mgr.initialize(
    prompt_idx=0,
    validate=True,
    strategy="compile",
    max_iterations=3
)

# 保存文件
mgr.save_example_rewritten("array_const", "main_rewritten.rs", code)
mgr.save_rewritten_file("main_rewritten.rs", code, is_final=True)

# 完成并统计
mgr.finalize(success_count, total_count, failed_examples)
```

#### refractor.py

生成和验证代码：

- `rewrite_file()` - 直接生成（无验证）
- `rewrite_file_with_validation()` - 迭代验证生成
- `extract_code()` - 从 LLM 响应提取代码

#### run_evaluation.sh

运行评估并复制结果到时间戳目录。

### 集成流程

```
run.sh
  ↓
refractor.py (OutputManager) ← 生成代码到时间戳目录
  ↓
run.sh 调用 run_evaluation.sh
  ↓
run_evaluation.sh
  ├─ compare_all.py ← 对比分析
  ├─ clippy_concurrency_eval.py ← 并发检查
  └─ 结果复制到时间戳目录
  ↓
输出显示完成
```

### 数据流

```
输入: examples/*/main.rs
↓
LLM 生成: main_rewritten_{N}.rs
↓
验证: 编译检查、安全检查等
↓
失败 → 反馈给 LLM → 重试
成功 → 保存到 result/{timestamp}/rewritten/
↓
评估: comparison_report.json
↓
最终: result/{timestamp}/ (完整输出)
```

### 文件组织原则

- **按时间戳组织** - 每次运行独立目录
- **按阶段保存** - round1, round2, ..., final
- **按示例分类** - examples/ 子目录
- **评估独立** - evaluation/ 子目录
- **配置记录** - config.json

### 向后兼容性

为保持与旧脚本兼容：

```bash
# 原有输出位置仍然被更新
examples/*/main_rewritten_{PROMPT_IDX}.rs ← 最新代码
result/{PROMPT_IDX}/*.json ← 最新评估结果
```

---

## 优化建议

### 对于快速运行

```bash
./run.sh 0 --no-validate --max-iterations 1
```

### 对于高质量输出

```bash
./run.sh 0 --validate --strategy comprehensive --max-iterations 5
```

### 对于批量处理

```bash
for prompt in 0 1 2; do
  ./run.sh $prompt --validate --strategy compile
done
```

### 对于持续集成

在 CI/CD 中运行：

```yaml
- name: Run LLM transformation
  run: cd LLM && ./run.sh 0 --validate --strategy compile
  
- name: Archive results
  if: always()
  run: |
    LATEST=$(ls result/ | sort -r | head -1)
    tar czf results_$LATEST.tar.gz result/$LATEST/
```

---

## 向后兼容性

所有旧命令继续工作：

```bash
# 旧方式（仍然有效）
python3 refractor.py 0
python3 validator.py examples/array_const/main_rewritten_0.rs
python3 cleanup.py --all --yes

# 新方式（推荐）
./run.sh 0 --validate --strategy compile
```

---

## 总结

✅ **特点**
- 时间戳自动组织
- 迭代过程完整保存
- 示例隔离输出
- 配置自动记录
- 完整流程集成
- 向后完全兼容

✅ **性能**
- 开销 < 1%
- 无额外延迟
- 磁盘占用 ~1-5MB per run

✅ **可靠性**
- 所有输出有备份
- 配置版本化记录
- 逐个示例可追踪

---

**最后更新: 2026年3月16日**
**系统状态: ✅ 生产就绪**
