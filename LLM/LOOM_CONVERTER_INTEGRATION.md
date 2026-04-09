# Loom Converter 与 Debug Validators 集成说明

## 概述

`debug_validators.py` 现已集成 `loom_converter.py`，可以在验证前自动将代码转换为 Loom 兼容格式。

## 集成工作流程

### 1. 运行单个 Loom 验证器

```bash
python3 -m validation.debug_validators <final.rs> --example-dir <dir> --validator loom
```

**工作流程：**
1. ✅ 调用 `_prepare_loom_test()` 方法
2. ✅ 执行 `loom_converter.py --standalone` 转换代码
3. ✅ 生成 `tests/loom.rs` 测试文件
4. ✅ 运行 Loom 验证器
5. ✅ 清理临时文件

### 2. 运行所有验证器

```bash
python3 -m validation.debug_validators <final.rs> --example-dir <dir>
```

**验证顺序：**
1. **Compile** - 编译检查
2. **Clippy** - 代码质量检查
3. **Miri** - 未定义行为检测
4. **Loom** - 并发问题检测（自动使用转换器）
5. **Safety** - 静态安全分析

## 集成的关键方法

### `_prepare_loom_test()`
自动调用 `loom_converter.py` 准备 Loom 测试：

```python
def _prepare_loom_test(self) -> Optional[str]:
    """
    Prepare loom test using loom_converter.
    
    Returns:
        Path to the loom test file if successful, None otherwise.
    """
```

**功能：**
- 定位 `loom_converter.py`（位于 LLM 目录）
- 创建 `tests/` 目录
- 调用转换器生成 `tests/loom.rs`
- 返回生成的文件路径

### `_cleanup_loom_test()`
清理生成的 Loom 测试文件：

```python
def _cleanup_loom_test(self, loom_test_path: Optional[str]) -> None:
```

## 实际示例

### 运行 array_const 的 Loom 验证

```bash
cd /home/guoxy/concrat/LLM

# 单个 loom 验证器
python3 -m validation.debug_validators validation/array_const/final.rs \
    --example-dir validation/array_const \
    --validator loom

# 完整验证流程
python3 -m validation.debug_validators validation/array_const/final.rs \
    --example-dir validation/array_const
```

### 预期输出

```
======================================================================
  4. LOOM VALIDATOR
======================================================================

Compilation passed, preparing loom concurrency test...

Preparing loom test using converter...
  Input: /home/guoxy/concrat/LLM/validation/array_const/final.rs
  Output: /home/guoxy/concrat/LLM/validation/array_const/tests/loom.rs
  Converter: /home/guoxy/concrat/LLM/loom_converter.py

  Step 1: Replacing concurrency primitives...
  Step 2: Converting main() to loom::model test...
  Converted file written to validation/array_const/tests/loom.rs
  Conversion complete!

✅ Loom test prepared successfully at validation/array_const/tests/loom.rs

Step 2: Running loom validation...

✅ PASS | loom
  Message: Loom analysis passed (no data races detected)
```

## 转换过程详解

当 Loom 验证器运行时，以下转换自动进行：

### 1. 并发原语替换
```rust
// 原始代码
use std::sync::{Arc, Mutex};
use std::thread;

// 转换后（仅在 loom 测试中）
use loom::sync::{Arc, Mutex};
use loom::thread;
```

### 2. main() 转换为测试
```rust
// 原始
fn main_0() { ... }
fn main() { unsafe { std::process::exit(main_0() as i32) } }

// 转换后
#[test]
fn test_concurrent_access() {
    loom::model(|| {
        // main_0 的内容
    });
}
```

## 关键文件

| 文件 | 位置 | 功能 |
|------|------|------|
| `debug_validators.py` | `/validation/` | 验证驱动脚本 |
| `loom_converter.py` | `/` (LLM 目录) | Loom 转换工具 |
| `LOOM_SETUP.md` | `/` | 转换工具详细文档 |

## 故障排除

### 问题：找不到 loom_converter.py

**解决方案：** 确保 `loom_converter.py` 在 LLM 根目录

```bash
ls -la /home/guoxy/concrat/LLM/loom_converter.py
```

### 问题：转换器超时

**解决方案：** 增加超时时间（脚本中默认为 10 秒）

### 问题：Loom 测试没有生成

**解决方案：** 检查 `tests/` 目录权限和磁盘空间

```bash
ls -la validation/array_const/tests/
```

## 性能注意事项

- ✅ 转换过程：< 1 秒
- ⚠️ Loom 测试执行：10-30 分钟（取决于代码复杂度）
- 💾 生成的测试文件：自动清理

## 支持的命令

| 命令 | 功能 |
|------|------|
| `--validator loom` | 仅运行 Loom 验证器 |
| `--validator compile` | 仅运行编译检查 |
| `--validator clippy` | 仅运行 Clippy 检查 |
| `--validator miri` | 仅运行 Miri 检查 |
| `--validator safety` | 仅运行安全分析 |
| （无选项） | 运行所有验证器 |

## 最佳实践

1. **先运行编译检查**
   ```bash
   python3 -m validation.debug_validators <file> --validator compile
   ```

2. **然后启用 Miri 检查**
   ```bash
   python3 -m validation.debug_validators <file> --validator miri
   ```

3. **最后运行 Loom（这会很慢）**
   ```bash
   python3 -m validation.debug_validators <file> --validator loom
   ```

## 参考

- 转换工具文档：[LOOM_SETUP.md](LOOM_SETUP.md)
- Loom 官方：https://github.com/tokio-rs/loom
- Validator 框架：[validation/core.py](validation/core.py)
