# 清理脚本使用指南 (cleanup.py)

## 概述

`cleanup.py` 是一个灵活的文件清理工具，用于管理生成的 `main_rewritten_*.rs` 文件。支持：
- 删除所有生成的文件
- 删除特定 prompt_idx 的文件
- 保留某个版本，删除其他所有版本
- 预览模式（--dry-run）
- 交互式和命令行两种操作方式

## 快速开始

### 1. 查看帮助
```bash
python3 cleanup.py --help
```

### 2. 预览所有生成的文件（不删除）
```bash
python3 cleanup.py --all --dry-run
```

### 3. 删除所有 main_rewritten_* 文件
```bash
python3 cleanup.py --all --yes
```

### 4. 删除特定版本
```bash
# 删除 main_rewritten_0.rs
python3 cleanup.py 0 --yes

# 删除 main_rewritten_0.rs 和 main_rewritten_1.rs
python3 cleanup.py 0 1 --yes

# 删除 main_rewritten_0/1/2.rs
python3 cleanup.py 0 1 2 --yes
```

### 5. 保留一个版本，删除其他所有
```bash
# 仅保留 main_rewritten_1.rs，删除 0/2/3
python3 cleanup.py --keep 1 --yes
```

### 6. 交互式模式
```bash
# 不带任何参数，进入交互式菜单
python3 cleanup.py
```

## 命令参数详解

| 参数 | 说明 | 示例 |
|------|------|------|
| `indices` | 删除指定的 prompt_idx | `python3 cleanup.py 0 1 2` |
| `--all` | 删除所有 main_rewritten_* | `python3 cleanup.py --all` |
| `--keep N` | 保留 prompt_idx=N，删除其他 | `python3 cleanup.py --keep 1` |
| `--pattern` | 自定义文件匹配模式 | `python3 cleanup.py --pattern "*.rs"` |
| `--dry-run` | 预览不删除 | `python3 cleanup.py --all --dry-run` |
| `--yes` | 跳过确认提示 | `python3 cleanup.py --all --yes` |
| `--examples-dir` | 指定 examples 目录 | `--examples-dir /path/to/examples` |

## 常见使用场景

### 场景 1: 快速预览磁盘空间使用

```bash
python3 cleanup.py --all --dry-run
```

**输出**:
```
找到 120 个文件，总大小 124.0KB

统计:
  main_rewritten_0.rs: 30 个文件
  main_rewritten_1.rs: 30 个文件
  main_rewritten_2.rs: 30 个文件
  main_rewritten_3.rs: 30 个文件
```

### 场景 2: 只保留最新的版本

你已经用 `--validate` 验证过 prompt_idx=1 的效果最好，现在要保留它并删除其他：

```bash
python3 cleanup.py --keep 1 --yes
```

**结果**: 删除 main_rewritten_0/2/3.rs，保留 main_rewritten_1.rs

### 场景 3: 删除失败的版本

假设 main_rewritten_0 的结果不好，要删除它：

```bash
python3 cleanup.py 0 --yes
```

**结果**: 删除所有 main_rewritten_0.rs 文件（30 个）

### 场景 4: 清空所有重新开始

```bash
python3 cleanup.py --all --yes
```

**结果**: 删除所有 120 个 main_rewritten_*.rs 文件

### 场景 5: 选择性保留多个版本

比如要删除 main_rewritten_0.rs 和 main_rewritten_3.rs，保留 1 和 2：

```bash
python3 cleanup.py 0 3 --yes
```

**结果**: 仅删除 prompt_idx=0 和 3 的文件

## 交互式模式

不带参数直接运行：

```bash
python3 cleanup.py
```

会看到菜单：

```
======================================================================
清理生成的 Rust 文件
======================================================================

选项:
  1. 删除所有 main_rewritten_*.rs
  2. 删除指定 prompt_idx 的文件 (例如只删 main_rewritten_0)
  3. 保留指定 prompt_idx，删除其他所有
  4. 预览所有 main_rewritten_* 文件
  0. 退出

请选择 [0-4]: 
```

### 交互式流程示例

```
请选择 [0-4]: 2

输入要删除的 prompt_idx (空格分隔, 例: 0 1 2): 0 2

删除 main_rewritten_{0, 2}.rs 文件
======================================================================

找到 60 个文件，总大小 62.5KB

...

确认删除? (yes/no): yes

删除文件中...

======================================================================
删除完成
======================================================================
✅ 成功删除: 60 个文件
📊 释放空间: 62.5KB
```

## 安全特性

### 1. 预览模式（--dry-run）
**推荐**: 先用 `--dry-run` 预览要删除的内容！

```bash
python3 cleanup.py --all --dry-run
```

### 2. 确认提示
未使用 `--yes` 时，删除前会询问确认：

```bash
确认删除? (yes/no): 
```

### 3. 错误处理
如果删除某个文件失败，会显示警告但继续处理其他文件：

```
⚠️  删除失败: /path/to/file - Permission denied
```

## 高级用法

### 自定义匹配模式

```bash
# 删除所有 .rs 文件
python3 cleanup.py --pattern "*.rs" --dry-run

# 删除 main.rs (原始文件)
python3 cleanup.py --pattern "main.rs" --yes
```

### 指定不同的 examples 目录

```bash
python3 cleanup.py --all --yes --examples-dir /path/to/examples
```

### 批量脚本中使用

```bash
#!/bin/bash
# 清理脚本
cd /home/guoxy/concrat/LLM

# 删除旧版本，只保留最新的
python3 cleanup.py --keep 2 --yes

# 验证删除结果
python3 cleanup.py --keep 2 --dry-run
```

## 输出说明

### 统计信息示例
```
找到 120 个文件，总大小 124.0KB

统计:
  main_rewritten_0.rs: 30 个文件      # prompt_idx=0 的版本
  main_rewritten_1.rs: 30 个文件      # prompt_idx=1 的版本
  main_rewritten_2.rs: 30 个文件      # prompt_idx=2 的版本
  main_rewritten_3.rs: 30 个文件      # prompt_idx=3 的版本
```

### 文件列表示例
```
文件列表 (前 20 个):
    1. [array_const         ] main_rewritten_0.rs            (543.0B)
    2. [array_const         ] main_rewritten_1.rs            (731.0B)
    3. [array_main          ] main_rewritten_0.rs            (667.0B)
    ...
  ... 和 100 个更多文件
```

显示：
- 序号
- 所在的示例名称
- 文件名
- 文件大小

### 删除完成报告
```
======================================================================
删除完成
======================================================================
✅ 成功删除: 60 个文件
❌ 删除失败: 0 个文件
📊 释放空间: 62.5KB
```

## 故障排除

### 问题 1: "Permission denied"

**原因**: 文件权限问题
**解决**: 用 `sudo` 运行（慎用）或检查文件权限

```bash
sudo python3 cleanup.py --all --yes
```

### 问题 2: 要删除的文件不存在

**症状**: 显示找到 0 个文件
**检查**:
1. 文件是否已被删除
2. 目录路径是否正确
3. 使用 `--examples-dir` 指定正确的路径

### 问题 3: 误删除了文件

**恢复**:
1. 如果有 git，可以恢复：
   ```bash
   cd /home/guoxy/concrat
   git checkout examples/*/main_rewritten_*.rs
   ```
2. 重新运行生成脚本

## 常用命令速查表

```bash
# 预览所有文件（推荐先做这个！）
python3 cleanup.py --all --dry-run

# 交互式模式
python3 cleanup.py

# 删除指定 prompt_idx
python3 cleanup.py 0 --yes

# 保留最新版本
python3 cleanup.py --keep 1 --yes

# 删除所有
python3 cleanup.py --all --yes

# 统计空间占用
python3 cleanup.py --all --dry-run | grep "大小"
```

## 配合迭代闭环使用

```bash
#!/bin/bash
# 迭代优化流程

cd /home/guoxy/concrat/LLM

# 第 1 步：生成 prompt_idx=0 的版本
python3 refractor.py 0 --validate --strategy compile

# 第 2 步：评估结果
python3 compare_all.py 0

# 第 3 步：尝试更激进的策略
python3 cleanup.py 0 --yes  # 删除旧版本
python3 refractor.py 1 --validate --strategy comprehensive

# 第 4 步：对比评估
python3 compare_all.py 1

# 第 5 步：保留最好的版本
python3 cleanup.py --keep 1 --yes
```

## 性能注意事项

- 删除 30 个文件: ~100ms
- 删除 120 个文件: ~300ms
- 预览模式速度相同

## 更新和扩展

如需修改清理策略，可编辑 `cleanup.py` 中的模式和逻辑。主要入口点：

- `find_files()`: 文件查找逻辑
- `count_by_type()`: 统计逻辑
- `main()`: 命令行接口

---

**版本**: 1.0
**状态**: ✅ 就绪
