# Output Verification Report

**Generated:** 2026-04-21T13:12:25.379102
**Total Examples:** 30
**Passed:** 7
**Failed:** 23

## Summary

- **Pass Rate:** 7/30 (23.3%)
- **Total Time:** 1216.0s
- **Average Time:** 40.5s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ Build failed | 0.0 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 2.4 |
| `array_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `global_assume2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_check` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 2.8 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 1.6 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 1.5 |
| `global_rwlock` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_trylock` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_alias` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_dup` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `struct_empty` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_init` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_main` | ❌ FAIL | ❌ Output mismatch | 0.8 |
| `struct_malloc` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_malloc2` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `struct_multiple` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_nested` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_spin` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_timedwait` | ❌ FAIL | ❌ Build failed | 0.0 |
| `unused_func` | ✅ PASS | ✅ Output verification passed | 0.9 |

## Failed Examples

### array_const

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_array_const_swayaeiy)
error[E0425]: cannot find function `pthread_mutex_lock` in this scope
   --> src/main.rs:110:5
    |
110 |     pthread_mutex_lock(
    |     ^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this function
    |
  1 + use libc::pthr
```

### array_main

```
Message: ❌ Output mismatch

Details:
Expected:
4 4 4 4 4

Actual:
2 2 2 2 2
```

### array_simple

```
Message: ❌ Build failed

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_array_simple_1hf8dqku)
error[E0277]: the trait bound `Arc<std::sync::Mutex<()>>: Copy` is not satisfied
  --> src/main.rs
```

### global_assume

```
Message: ❌ Output mismatch

Details:
Expected:
2

Actual:
0
```

### global_assume2

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_assume2_s14ck64i)
warning: unused import: `Arc`
 --> src/main.
```

### global_check

```
Message: ❌ Build failed

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_check_o8cti9ka)
error[E0596]: cannot borrow data in an `Arc` as mutable
  --> src/main.rs:25:5
   |
25 |     mutex
```

### global_condvar

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### global_custom

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### global_main

```
Message: ❌ Output mismatch

Details:
Expected:
4

Actual:
3
```

### global_rwlock

```
Message: ❌ Build failed

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_rwlock_ro9cu_71)
warning: unused import: `std::sync::Once`
 --
```

### global_simple

```
Message: ❌ Build failed

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_simple_83mgc5uv)
error[E0015]: cannot call non-const associated function `Arc::<std::sync::Mutex<i32>>::new` in st
```

### struct_alias

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_alias_dm3_tqad)
error[E0015]: cannot call non-const associated function `Arc::<std::sync::Mutex<St>>::new` in statics
  --> src/main.rs:16:33
   |
16 |   pub static S1: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
   |  _________________________________^
17 | |     n1: 0,

```

### struct_condvar

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### struct_dup

```
Message: ❌ Output mismatch

Details:
Expected:
8 1
10 3

Actual:
0 1
2 3
```

### struct_empty

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_empty_mtgq3t0f)
error[E0015]: cannot call non-const associated function `Arc::<std::sync::Mutex<Ss>>::new` in statics
  --> src/main.rs:11:29
   |
11 |   static S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
   |  _____________________________^
12 | |     n: 0,
13 | |   
```

### struct_init

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_init_jccc5nrh)
error[E0015]: cannot call non-const associated function `Arc::<std::sync::Mutex<Ss>>::new` in statics
  --> src/main.rs:12:29
   |
12 | static S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));
   |                             ^^^^
```

### struct_main

```
Message: ❌ Output mismatch

Details:
Expected:
4

Actual:
3
```

### struct_malloc

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc_zh9tz1ru)
error[E0594]: cannot assign to data in an `Arc`
  --> src/main.rs:14:5
   |
14 |     s.n += 1;
   |     ^^^^^^^^ cannot assign
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<S>`

warning:
```

### struct_multiple

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### struct_nested

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_nested_78u17mc0)
warning: unused import: `std::thread`
 --> src/main.rs:2:5
  |
2 | use std::thread;
  |     ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::ffi::CString`
 --> src/main.rs:3:5
 
```

### struct_simple

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_simple_x8wxper3)
error[E0382]: use of moved value: `shared_data`
  --> src/main.rs:57:33
   |
54 |     let shared_data = init_shared_data();
   |         ----------- move occurs because `shared_data` has type `Arc<std::sync::Mutex<SharedData>>`, which does not implement
```

### struct_spin

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_spin_103z0dto)
error[E0308]: mismatched types
  --> src/main.rs:40:31
   |
40 |     let s_clone1 = Arc::clone(&s);
   |                    ---------- ^^ expected `&Arc<_, _>`, found `&Ss`
   |                    |
   |                    arguments to this function are i
```

### struct_timedwait

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_timedwait_k4bo4sd0)
error[E0599]: no method named `wait_until` found for struct `std::sync::Condvar` in the current scope
  --> src/main.rs:24:33
   |
24 |         let result = state.cond.wait_until(&mut guard, || Instant::now() < wait_until).unwrap();
   |             
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `global_nested` | 1.6 |
| `global_read` | 1.5 |
| `global_trylock` | 1.0 |
| `global_while` | 1.3 |
| `struct_assume` | 0.9 |
| `struct_malloc2` | 1.0 |
| `unused_func` | 0.9 |
