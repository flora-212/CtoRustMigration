# Concrat Testing & Metrics Comparison

## 🔍 测试和指标分类

---

## ✅ **仓库原有的测试（Official Tests）**

### 1️⃣ **基础功能测试（Basic Testing Scripts）**

#### `test_examples.sh`
**用途**: 测试所有小型示例项目
```bash
./scripts/test_examples.sh
```
- 遍历 `examples/` 目录（34个示例）
- 对每个示例执行完整的分析→转换→编译流程
- 验证转换是否成功

#### `test_dataflow.sh`
**用途**: 测试数据流分析准确性
```bash
./scripts/test_dataflow.sh
```
- 遍历 `dataflow_examples/` 目录（17个示例）
- 运行数据流分析并与预期结果对比（`b.json`）
- 验证锁分析的正确性

#### `test_bench_*.sh`
**用途**: 测试大规模真实项目
```bash
./scripts/test_bench_success.sh    # 预期成功的项目
./scripts/test_bench_fail.sh       # 预期失败的项目（用于回归测试）
```

---

### 2️⃣ **性能和规模指标（Performance & Scale Metrics）**

#### `experiment_transform.sh` ⭐
**用途**: 测量转换性能和代码变化
```bash
./scripts/experiment_transform.sh
```

**输出指标**:
```
Ana     Tra     Ins     Del     Succ    Project
0.234   0.156   +23     -18     yes     pigz
```

**指标解释**:
- `Ana` (Analysis Time): 数据流分析耗时（秒）
- `Tra` (Translation Time): 代码转换耗时（秒）
- `Ins` (Insertions): 插入的代码行数
- `Del` (Deletions): 删除的代码行数
- `Succ` (Success): 转换是否成功

**意义**: 
- 评估工具的性能开销
- 量化代码变化程度
- 跟踪性能回归

#### `experiment_count.sh` ⭐
**用途**: 统计代码规模和并发API使用
```bash
./scripts/experiment_count.sh
```

**输出指标**:
```
C       Rust    Mutex   Rwlock  Spin    Cond    Project
1234    2345    5       2       0       1       pigz
```

**指标解释**:
- `C`: C源码行数（使用 `cloc` 工具）
- `Rust`: Rust代码行数（使用 `wc -l`）
- `Mutex, Rwlock, Spin, Cond`: 不同并发原语的使用次数（使用 `api_counter` 工具）

**依赖工具**:
- `cloc`: 代码行数统计工具（需要安装）
- `api_counter`: 自定义工具，统计并发API使用次数（**仓库中未包含源码**）

**意义**: 
- 评估项目规模
- 统计并发模式的分布
- 对比不同项目的复杂度

#### `experiment_analyze.sh`
**用途**: 测量静态分析（Goblint）的耗时
```bash
./scripts/experiment_analyze.sh
```

**输出指标**:
```
Time    Project
2.345   pigz
```

**意义**: 评估 Goblint 分析的性能

---

### 3️⃣ **正确性验证（Correctness Verification）**

#### `experiment_correctness.sh` ⭐
**用途**: 验证转换保持行为等价性
```bash
./scripts/experiment_correctness.sh
```

**测试流程**:
```bash
# 每个项目测试三个阶段：
1. make test C=yes      # C代码能否运行？
2. make test            # 初始Rust代码能否运行？
3. make transformation  # 转换后Rust代码能否运行？
   make test
   make revert
```

**输出指标**:
```
C       Rust    Trans   Project
SUCC    SUCC    SUCC    pigz
SUCC    SUCC    FAIL    kona     # 转换引入了bug
```

**意义**: 
- 确保语义保持不变
- 检测转换引入的回归
- 验证功能等价性

---

## 🆕 **我新引入的工具（My Contribution）**

### `safety_checker.sh` ⭐⭐⭐
**用途**: 量化安全性改进（Safety Improvement Quantification）
```bash
./safety_checker.sh before.rs after.rs
```

**输出示例**:
```
================================================
  Concrat Safety Improvement Analysis
================================================

📊 Unsafe Code Metrics:
  ┌─────────────────────────────┬────────┬────────┬──────────┐
  │ Metric                      │ Before │ After  │ Change   │
  ├─────────────────────────────┼────────┼────────┼──────────┤
  │ static mut (global vars)    │      4 │      2 │       +2 │
  │ unsafe fn                   │      6 │      6 │       +0 │
  │ pthread_mutex_* calls       │      4 │      2 │       +2 │
  └─────────────────────────────┴────────┴────────┴──────────┘

✅ Safe Rust Concurrency Primitives (After):
  │ Mutex<T>                    │      1 │
  │ RwLock<T>                   │      0 │
  │ .lock() calls               │      1 │

🎯 Key Safety Improvement:
  • Reduced mutable global statics by: 50.0%
  • Eliminated unsafe pthread calls: 2
  • Introduced type-safe synchronization primitives
```

**新增指标**:
1. **`static mut` 数量**: 全局可变静态变量（Rust中最危险的特性之一）
2. **`unsafe fn` 数量**: 不安全函数
3. **`pthread_mutex_*` 调用**: 手动锁管理（应该被消除）
4. **`Mutex<T>` 使用**: 类型安全的互斥锁
5. **`RwLock<T>` 使用**: 类型安全的读写锁
6. **`.lock()` 调用**: RAII风格的锁获取

**为什么这些指标重要**:
- ❌ `static mut` → 无编译器保护的全局可变状态
- ❌ `pthread_mutex_*` → 手动管理，容易忘记解锁
- ✅ `Mutex<T>` → 编译器强制互斥访问
- ✅ `.lock()` → RAII自动释放锁

**与原有工具的区别**:
| 特性 | `experiment_count.sh` | `safety_checker.sh` |
|------|----------------------|---------------------|
| 统计Mutex数量 | ✅ (需要api_counter) | ✅ (仅需grep) |
| 对比转换前后 | ❌ | ✅ |
| 量化安全改进 | ❌ | ✅ |
| 统计unsafe代码 | ❌ | ✅ |
| 计算改进百分比 | ❌ | ✅ |
| 依赖外部工具 | ✅ (api_counter, cloc) | ❌ |

---

## 📊 **指标详解与意义**

### 🔴 **危险代码指标（Unsafe Code Metrics）**

#### 1. `static mut` (Mutable Static Variables)
```rust
// ❌ DANGER: 无任何保护
pub static mut counter: i32 = 0;

unsafe {
    counter += 1;  // 数据竞争！编译器无法阻止
}
```

**为什么危险**:
- 全局可变状态
- 多线程访问会导致数据竞争
- 需要 `unsafe` 才能访问
- 编译器无法提供保护

**Concrat的改进**:
```rust
// ✅ SAFE: 编译器强制保护
pub static counter: Mutex<i32> = Mutex::new(0);

let mut guard = counter.lock().unwrap();  // 必须获取锁
*guard += 1;  // 编译器保证互斥访问
```

#### 2. `unsafe fn` (Unsafe Functions)
```rust
pub unsafe extern "C" fn process() {
    // 调用者需要确保满足安全前提
}
```

**为什么需要追踪**: 
- `unsafe` 代码绕过了 Rust 安全检查
- 越少越好
- Concrat应该减少 `unsafe fn` 的使用

#### 3. `pthread_mutex_*` 调用 (Manual Lock Operations)
```rust
pthread_mutex_lock(&mut m);    // ❌ 手动管理
// ... 如果这里 panic，锁永远不会释放！
pthread_mutex_unlock(&mut m);  // ❌ 容易忘记
```

**问题**:
- 需要手动配对 lock/unlock
- 异常退出时不会自动解锁 → 死锁
- 编译器无法验证正确性

---

### 🟢 **安全代码指标（Safe Code Metrics）**

#### 4. `Mutex<T>` (Type-Safe Mutex)
```rust
pub static data: Mutex<Data> = Mutex::new(Data { ... });

let guard = data.lock().unwrap();  // 获取 MutexGuard<Data>
// guard 析构时自动释放锁 (RAII)
```

**优势**:
- 类型系统保证：只能通过锁访问数据
- RAII自动管理生命周期
- 编译时防止数据竞争
- 无法忘记释放锁

#### 5. `RwLock<T>` (Reader-Writer Lock)
```rust
pub static cache: RwLock<HashMap<K, V>> = ...;

let read_guard = cache.read().unwrap();   // 多个读者
let write_guard = cache.write().unwrap(); // 独占写入
```

**适用场景**: 读多写少的数据

#### 6. `.lock()` / `.read()` / `.write()` 调用
统计这些调用可以了解：
- 锁的使用频率
- 潜在的性能瓶颈
- 并发粒度

---

### 📈 **性能指标（Performance Metrics）**

#### 7. 分析时间 (Analysis Time)
- 测量数据流分析的耗时
- 评估工具的可扩展性
- 对于大型项目（100K+ LOC）很重要

#### 8. 转换时间 (Translation Time)
- 测量代码重写的耗时
- 通常比分析快得多

#### 9. 代码变化量 (Code Changes)
- `Ins` (插入行): 新增的代码（如锁保护代码）
- `Del` (删除行): 移除的代码（如pthread调用）
- 净变化: `Ins - Del`

**典型模式**:
```
Ins: +20   # 添加了 Mutex<T> 包装和 lock() 调用
Del: -15   # 删除了 pthread_mutex_* 调用
净: +5     # 代码稍微变长（类型安全的代价）
```

---

## 🎯 **总结对比表**

| 测试/指标 | 类型 | 来源 | 目的 | 关键价值 |
|----------|------|------|------|----------|
| `test_*.sh` | 功能测试 | 原仓库 | 验证转换成功 | 基础保证 |
| `experiment_correctness.sh` | 正确性 | 原仓库 | 验证行为等价 | 防止回归 |
| `experiment_transform.sh` | 性能 | 原仓库 | 测量时间/代码变化 | 评估开销 |
| `experiment_count.sh` | 规模 | 原仓库 | 统计LOC和API | 了解分布 |
| `experiment_analyze.sh` | 性能 | 原仓库 | 测量Goblint时间 | 优化瓶颈 |
| **`safety_checker.sh`** | **安全性** | **新增** | **量化安全改进** | **核心价值** |

---

## 💡 **为什么需要 safety_checker.sh**

原仓库的测试主要关注：
1. ✅ **功能正确性** - 代码能编译、能运行
2. ✅ **性能开销** - 分析和转换的时间
3. ✅ **规模统计** - 项目大小、API分布

但是**缺失了最重要的一环**:
4. ❌ **安全性改进** - 代码变得更安全了吗？

**Concrat 的核心价值主张是什么？**
> 将不安全的并发C代码转换为**类型安全**的Rust代码

但是原有测试**没有直接测量这一点**！

**`safety_checker.sh` 填补了这个空白**:
- 📊 量化 `unsafe` 代码的减少
- 📊 展示类型安全原语的引入
- 📊 计算安全性改进百分比
- 📊 提供清晰的安全性报告

这对于：
- **研究论文**: 需要数据支持"更安全"的声明
- **用户说服**: 展示实际的安全性收益
- **工具评估**: 对比不同版本的改进

都是**至关重要**的指标！

---

## 🚀 **使用建议**

### 运行完整测试套件：
```bash
# 1. 功能测试
./scripts/test_examples.sh

# 2. 正确性验证
./scripts/experiment_correctness.sh

# 3. 性能评估
./scripts/experiment_transform.sh

# 4. 安全性量化（新增）
for dir in examples/*/; do
    cp "$dir/main.rs" /tmp/before.rs
    ./scripts/test_dir.sh "$dir" /tmp/test_out
    /tmp/safety_checker.sh /tmp/before.rs /tmp/test_out/main.rs
done
```

### 生成研究论文数据：
```bash
# 表格1: 项目规模和API分布
./scripts/experiment_count.sh > results/scale.tsv

# 表格2: 性能开销
./scripts/experiment_transform.sh > results/performance.tsv

# 表格3: 安全性改进（新增）
./scripts/batch_safety_check.sh > results/safety.tsv
```
