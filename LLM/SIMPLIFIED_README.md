# Final Consistency Check Script - 简化版本

## 概述

简化后的脚本只提供三个模式，全部面向failed examples：
1. **check** - 检查所有examples的一致性
2. **update-max-round** - 更新failed examples为最大round
3. **update-c2rust** - 更新failed examples为c2rust

## 快速开始

```bash
cd /home/guoxy/concrat/LLM

# 1. 检查一致性（所有62个examples）
./check_final_consistency.sh check /path/to/result

# 2. 只更新编译失败的examples为最新round
./check_final_consistency.sh update-max-round /path/to/result

# 3. 只更新编译失败的examples为c2rust
./check_final_consistency.sh update-c2rust /path/to/result
```

## 工作原理

### Check 模式
检查所有examples的final.rs是否与最大round或c2rust代码一致。

**输出示例:**
```
✓ array_const: final = c2rust
! array_const____deadlock: final ≠ max_round 且 ≠ c2rust
✓ array_main: final = max_round (round20)
```

### Update-max-round 模式
从config.json中读取failed_examples列表，只更新这些examples的final.rs为对应的最大round代码。

**工作流程:**
1. 读取result目录中的config.json
2. 提取failed_examples列表
3. 遍历该列表中的每个example
4. 将其final.rs替换为最大round代码

**输出示例:**
```
Failed examples: 30

→ array_const: 更新为 round20
✓ array_const____lock_mismatch: 已经是最大round
→ array_main: 更新为 round20
```

### Update-c2rust 模式
从config.json中读取failed_examples列表，只更新这些examples的final.rs为对应的c2rust代码。

## 关键特性

✓ **精准更新** - 只更新编译失败的examples，不触及编译通过的  
✓ **忽略空行** - 文件比较时自动忽略空行差异  
✓ **智能重命名** - 支持带下划线后缀的例子名称解析  
✓ **彩色输出** - 便于快速查看结果  
✓ **错误统计** - 详细的处理统计信息  

## 脚本位置

- 主脚本：`/home/guoxy/concrat/LLM/check_final_consistency.sh`

## 使用场景

### 场景1: 检查当前状态
想要了解62个examples中有多少与最大round或c2rust一致

```bash
./check_final_consistency.sh check /home/guoxy/concrat/LLM/result/20260412_155953_6_compile
```

输出会显示每个example的状态和统计信息。

### 场景2: 修复编译失败的examples
编译失败了部分examples（由config.json记录），想要将其final.rs更新为最新的迭代版本

```bash
./check_final_consistency.sh update-max-round /home/guoxy/concrat/LLM/result/20260412_155953_6_compile
```

只会处理failed_examples中的examples，其他examples保持不变。

### 场景3: 重置编译失败的examples
想要将编译失败的examples重置为初始的c2rust版本

```bash
./check_final_consistency.sh update-c2rust /home/guoxy/concrat/LLM/result/20260412_155953_6_compile
```

只会处理failed_examples中的examples。

## 文件比较逻辑

脚本使用以下流程比较文件：

1. 读取两个文件
2. 使用 `grep -v '^$'` 删除所有空行
3. 使用 `diff -q` 比较删除空行后的内容
4. 如果内容相同，则认为两个文件一致

这意味着只要代码内容相同，空行数量不同也不影响。

## 输出符号说明

| 符号 | 颜色 | 含义 |
|------|------|------|
| ✓ | 绿色 | 一致/成功/已是目标版本 |
| ! | 黄色 | 不一致/警告 |
| ✗ | 红色 | 错误/文件不存在 |
| → | 黄色 | 正在更新 |

## 版本历史

### v3.0 - 简化版本 ✨ 当前版本
- 移除了全量更新模式
- 所有update模式现在都只针对failed examples
- 使用更简洁的模式名称

### v2.0 - 添加update-failed模式
- 添加了只更新failed examples的功能
- 保留了全量更新模式以供选择

### v1.0 - 初版
- 基本的检查功能
- 忽略空行差异
