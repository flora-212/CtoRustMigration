# Output Verification Report

**Generated:** 2026-04-21T13:07:08.475641
**Total Examples:** 30
**Passed:** 6
**Failed:** 24

## Summary

- **Pass Rate:** 6/30 (20.0%)
- **Total Time:** 657.4s
- **Average Time:** 21.9s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Output verification passed | 4.4 |
| `array_main` | ❌ FAIL | ❌ Build failed | 0.0 |
| `array_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 1.3 |
| `global_assume2` | ❌ FAIL | ❌ Output mismatch | 3.0 |
| `global_check` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 4.2 |
| `global_nested` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 3.1 |
| `global_rwlock` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 6.9 |
| `global_trylock` | ❌ FAIL | ❌ Output mismatch | 20.9 |
| `global_while` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_alias` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 3.9 |
| `struct_condvar` | ❌ FAIL | ❌ Output mismatch | 3.0 |
| `struct_dup` | ❌ FAIL | ❌ Output mismatch | 2.5 |
| `struct_empty` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_init` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_main` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_malloc` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_malloc2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_multiple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 2.8 |
| `struct_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_spin` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_timedwait` | ❌ FAIL | ❌ Build failed | 0.0 |
| `unused_func` | ✅ PASS | ✅ Output verification passed | 1.6 |

## Failed Examples

### array_main

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compi
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
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_array_simple_8nyz2151)
error[E0015]: cannot call non-const associated
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
Message: ❌ Output mismatch

Details:
Expected:
2

Actual:
0
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
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_check_yz88fkcx)
error[E0015]: cannot call non-const associated
```

### global_condvar

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### global_custom

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_global_custom_wvw
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

### global_nested

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_nested_anx
```

### global_rwlock

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_rwlock_kl7
```

### global_trylock

```
Message: ❌ Output mismatch

Details:
Expected:
8

Actual:
0
```

### global_while

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_while_bdxc
```

### struct_alias

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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_alias_25_4b_ji)
error[E0308]: mismatched types
  --> src/main.
```

### struct_condvar

```
Message: ❌ Output mismatch

Details:
Expected:
2

Actual:
0
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
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_empty_8z9t
```

### struct_init

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_init_78rx4
```

### struct_main

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### struct_malloc

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc_a2xgv32q)
error[E0308]: mismatched types
  --> src/main.rs:18:18
   |
18 |         s.c.wait(&mut guard);
   |             ---- ^^^^^^^^^^ expected `MutexGuard<'_, _>`, found `&mut MutexGuard<'_, ()>`
   |             |
   |             arguments to this method ar
```

### struct_malloc2

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc2_hr8u83tn)
warning: unused import: `Arc`
 --> src/main.rs:1:17
  |
1 | use std::sync::{Arc, Mutex};
  |                 ^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error[E0308]: mismatched types
   --> src/main.rs:61:22

```

### struct_multiple

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_multiple_mp_waxl1)
error[E0277]: `St` doesn't implement `Debug`
  --> src/main.rs:23:13
   |
23 |         })).unwrap();
   |             ^^^^^^ the trait `Debug` is not implemented for `St`
   |
   = note: add `#[derive(Debug)]` to `St` or manually `impl Debug for St`
h
```

### struct_simple

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_sim
```

### struct_spin

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
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_spin_cpoko36i)
error[E0308]: mismatched types
  --> src/main.r
```

### struct_timedwait

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_tim
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 4.4 |
| `global_read` | 3.1 |
| `global_simple` | 6.9 |
| `struct_assume` | 3.9 |
| `struct_nested` | 2.8 |
| `unused_func` | 1.6 |
