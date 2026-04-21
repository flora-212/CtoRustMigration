# 脚本更新日志

## 最新更新（v2.0）- 添加update-failed模式

### 新功能

添加了 `update-failed` 模式，可以**只更新编译不通过的examples**（来自config.json中的failed_examples列表）

```bash
./check_final_consistency.sh update-failed /path/to/result
```

### 更新原因

- 原来的update-max-round会更新所有final.rs文件
- 现在可以指定只更新failed_examples，节省时间且更加精准

### 工作原理

1. 读取result目录中的config.json
2. 提取`failed_examples`列表（编译不通过的examples）
3. 只对这些example的final.rs进行更新
4. 其他successful examples保持不变

### 脚本位置

- `/home/guoxy/concrat/LLM/check_final_consistency.sh`

### 使用方式

```bash
cd /home/guoxy/concrat/LLM

# 1. 检查一致性（所有examples）
./check_final_consistency.sh check /path/to/result

# 2. 更新所有final.rs为最大round
./check_final_consistency.sh update-max-round /path/to/result

# 3. 只更新编译不通过的examples为最大round ✨ 新增
./check_final_consistency.sh update-failed /path/to/result

# 4. 更新所有final.rs为c2rust代码
./check_final_consistency.sh update-c2rust /path/to/result
```

### 测试结果（20260417_200854_6_compile）

#### 执行 `update-failed` 前
```
总共检查例子数: 62
  最大round一致: 39
  c2rust一致: 12
  不一致: 11
```

#### 执行 `update-failed` 后
```
Failed examples count: 23
总共处理例子数: 23  (只处理failed examples)

✓ array_const: 已经是最大round
→ array_const____lock_mismatch: 更新为 round20
→ array_main____lock_leak: 更新为 round20
→ array_simple: 更新为 round20
... (共23个failed examples)
```

#### 验证后
```
总共检查例子数: 62
  最大round一致: 62 ✓ (从39增加到62)
  c2rust一致: 0
  不一致: 0
```

### 主要改进

| 指标 | 之前 | 现在 |
|------|------|------|
| 检查一致性模式 | ✓ | ✓ |
| 忽略空行差异 | ✓ | ✓ |
| 批量更新所有 | ✓ | ✓ |
| **只更新failed** | ✗ | **✓ 新增** |
| 处理精准度 | 低 | **高** |

---

## 上一个更新（v1.0）- 忽略空行差异

原来的比较方式使用 `cmp -s` 逐字节比较，会将空行差异认为是不一致。

已改为使用 `grep -v '^$'` 删除所有空行后再进行比较，使得只要代码内容相同就认为一致。
