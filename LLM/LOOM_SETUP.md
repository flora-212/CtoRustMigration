# Loom 并发测试集成指南

## 概述

此指南说明如何使用 `loom_converter.py` 工具将 C2Rust 生成的代码转换为 Loom 并发测试格式。

## 快速开始

### 1. 生成库模块版本（推荐用于现有项目）

```bash
cd /home/guoxy/concrat/LLM
python3 loom_converter.py validation/array_const/final.rs \
                         validation/array_const/main.rs \
                         --example-dir validation/array_const
```

**特点：**
- `main.rs` 仍作为库模块使用
- 使用 `#[cfg(loom)]` 条件编译
- 不中断现有构建系统
- 正常编译用 `std::sync`，loom 测试用 `loom::sync`

### 2. 生成独立 Loom 测试

```bash
python3 loom_converter.py validation/array_const/final.rs \
                         validation/array_const/tests/loom.rs \
                         --example-dir validation/array_const \
                         --standalone
```

**特点：**
- 创建 `tests/loom.rs` 集成测试
- `main()` 自动转换为 `#[test]` 函数
- 所有并发原语使用 Loom 版本
- 独立运行，不影响主库

## 转换细节

### 并发原语映射

| 原始 | → | Loom |
|------|---|------|
| `std::sync::Arc` | → | `loom::sync::Arc` |
| `std::sync::Mutex` | → | `loom::sync::Mutex` |
| `std::thread::spawn` | → | `loom::thread::spawn` |
| `use std::sync::` | → | `use loom::sync::` |

### Main 函数转换（仅 --standalone）

**输入：**
```rust
fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| { ... });
    let handle2 = thread::spawn(|| { ... });
    handle1.join().unwrap();
    handle2.join().unwrap();
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
```

**输出：**
```rust
#[test]
fn test_concurrent_access() {
    loom::model(|| {
        let handle1 = loom::thread::spawn(|| { ... });
        let handle2 = loom::thread::spawn(|| { ... });
        handle1.join().unwrap();
        handle2.join().unwrap();
    });
}
```

## 运行 Loom 测试

### 标准库模块测试

```bash
cd validation/array_const
cargo build                    # 普通编译（使用 std::sync）
RUSTFLAGS="--cfg loom" cargo build --release  # Loom 编译
```

### 独立测试运行

```bash
cd validation/array_const
RUSTFLAGS="--cfg loom" cargo test --test loom --release -- --nocapture
```

## 配置要求

### Cargo.toml

工具会自动添加到 `[target.'cfg(loom)'.dependencies]`:

```toml
[dependencies]
libc = "0.2"
lazy_static = "1.4"

[target.'cfg(loom)'.dependencies]
loom = "0.7"
```

## 性能注意事项

- **非常慢**：Loom 通过排列 C11 内存模型执行，可能需要数分钟甚至数小时
- **超时**：编译器默认 60 分钟超时
- **优化**：使用 `--release` 标志加快测试（10-30 倍快）
- **单一模式**：运行时环境变量 `LOOM_CHECKPOINT_INTERVAL=10000` 可加快

示例：
```bash
RUSTFLAGS="--cfg loom" cargo test --test loom --release -- --nocapture
```

## 已知问题

### 1. lazy_static 与 Loom

`lazy_static` 全局变量在 Loom 的受限环境中可能表现不同：

```rust
// 可能有问题
lazy_static::lazy_static! {
    static ref GLOBAL: Arc<Mutex<Data>> = Arc::new(Mutex::new(Data::new()));
}
```

**解决方案**：
- 在 loom 测试中避免使用 global statics
- 使用 `loom::sync::Mutex` 的本地变量替代
- 或使用 `#[cfg(not(loom))]` 条件忽略 global statics

### 2. Unsafe 阻塞操作

```rust
// 在 Loom 中仍然 unsafe
unsafe extern "C" fn f1() {
    NUM_MUTEX[0].lock();
    // ...
}
```

Loom 无法跟踪 unsafe 块中的同步原语。如果可能，在可测试代码中避免使用它。

## 工具源代码

- **位置**：`/home/guoxy/concrat/LLM/loom_converter.py`
- **功能**：
  1. 使用正则表达式和括号匹配进行 AST 级别的代码转换
  2. 支持条件编译的库模块
  3. 独立测试生成
  4. 自动 Cargo.toml 更新

## 示例工作流

### 对现有 array_const 示例进行 Loom 测试

```bash
# 1. 进入项目目录
cd /home/guoxy/concrat/LLM

# 2. 运行转换（库模式）
python3 loom_converter.py validation/array_const/final.rs \
                         validation/array_const/main.rs

# 3. 验证普通编译
cd validation/array_const
cargo build

# 4. 生成 loom 测试
cd /home/guoxy/concrat/LLM
python3 loom_converter.py validation/array_const/final.rs \
                         validation/array_const/tests/loom.rs \
                         --standalone

# 5. 运行 loom 测试
cd validation/array_const
RUSTFLAGS="--cfg loom" cargo test --test loom --release
```

## 参考资源

- **Loom 官方**：https://github.com/tokio-rs/loom
- **Loom 文档**：https://docs.rs/loom
- **Tokio**：https://tokio.rs/

## 故障排除

| 问题 | 原因 | 解决方案 |
|------|------|--------|
| `Arc leaked` | 内存管理问题 | 检查引用计数，确保所有 Arc 被正确释放 |
| 编译失败（未找到 loom） | 缺少 cfg 标志 | 使用 `RUSTFLAGS="--cfg loom"` |
| 测试超时 | Loom 排列过多 | 简化测试或增加超时间隔 |
| 条件编译警告 | 未知 cfg | 更新 Cargo.toml 的 lints 或 build.rs |
