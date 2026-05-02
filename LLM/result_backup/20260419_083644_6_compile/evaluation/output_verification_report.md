# Output Verification Report

**Generated:** 2026-04-21T13:10:28.296329
**Total Examples:** 30
**Passed:** 8
**Failed:** 22

## Summary

- **Pass Rate:** 8/30 (26.7%)
- **Total Time:** 630.7s
- **Average Time:** 21.0s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ Build failed | 0.0 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 5.1 |
| `array_simple` | ✅ PASS | ✅ Output verification passed | 2.2 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 1.9 |
| `global_assume2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_check` | ✅ PASS | ✅ Output verification passed | 2.5 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 1.3 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `global_rwlock` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_trylock` | ❌ FAIL | ❌ Output mismatch | 1.2 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `struct_alias` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_assume` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_condvar` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_dup` | ✅ PASS | ✅ Output verification passed | 1.2 |
| `struct_empty` | ❌ FAIL | ❌ Output mismatch | 1.7 |
| `struct_init` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_main` | ❌ FAIL | ❌ Output mismatch | 1.1 |
| `struct_malloc` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_malloc2` | ❌ FAIL | ❌ Output mismatch | 2.4 |
| `struct_multiple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_simple` | ❌ FAIL | ❌ Output mismatch | 1.3 |
| `struct_spin` | ❌ FAIL | ❌ Output mismatch | 1.3 |
| `struct_timedwait` | ❌ FAIL | ❌ Build failed | 0.0 |
| `unused_func` | ❌ FAIL | ❌ Output mismatch | 1.2 |

## Failed Examples

### array_const

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
   Compiling output_test v0.1.0 (/tmp/output_test_array_const_k4kyy
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
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_global_assume2_1pfdhum3)
warning: unused import: `Arc`
 --> src/main.
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
0
```

### global_rwlock

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_rwlock_zt98934g)
error[E0015]: cannot call non-const associated function `RwLock::new` in statics
  --> src/main.rs:32:23
   |
32 | static LOCK: RwLock = RwLock::new();
   |                       ^^^^^^^^^^^^^
   |
   = note: calls in statics are limited to constant fun
```

### global_simple

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_global_simple_4ngn0d49)
error[E0599]: no method named `lock` found for struct `Arc<SharedData>` in the current scope
  --> src/main.rs:14:32
   |
14 |         let data = shared_data.lock().unwrap();
   |                                ^^^^ method not found in `Arc<SharedData>`
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

### struct_assume

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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_assume_xn9i9htn)
error[E0308]: mismatched types
  --> src/main.rs:42:19
   |
42 |             t_fun(s_ptr);
   |  
```

### struct_condvar

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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_condvar_sg4tctuv)
warning: unused import: `std::ffi::CStr`
 --
```

### struct_empty

```
Message: ❌ Output mismatch

Details:
Expected:
0 0

Actual:

```

### struct_init

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_init_84x5c39c)
error[E0308]: mismatched types
  --> src/main.rs:31:8
   |
31 |     f1(&mut S1.lock().unwrap());
   |     -- ^^^^^^^^^^^^^^^^^^^^^^^ expected `*mut Ss`, found `&mut MutexGuard<'_, Ss>`
   |     |
   |     arguments to this function are incorrect
   |
   =
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
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc_xyv8aphe)
warning: unused import: `std::ffi::CSt
```

### struct_malloc2

```
Message: ❌ Output mismatch

Details:
Expected:
789

Actual:

```

### struct_multiple

```
Message: ❌ Build failed

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_multiple_aqe3_xy0)
error[E0432]: unresolved import `std::sync::Lazy`
 --> src/main.rs:4:5
  |
4 | use std::sync::L
```

### struct_simple

```
Message: ❌ Output mismatch

Details:
Expected:
6 7 8 3

Actual:
0 1 2 3
```

### struct_spin

```
Message: ❌ Output mismatch

Details:
Expected:
3 4 5

Actual:
1 2 3
```

### struct_timedwait

```
Message: ❌ Build failed

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_timedwait_2hwrttwk)
warning: unused import: `UNIX_EPOCH
```

### unused_func

```
Message: ❌ Output mismatch

Details:
Expected:
2

Actual:
0
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_simple` | 2.2 |
| `global_check` | 2.5 |
| `global_nested` | 1.3 |
| `global_read` | 1.3 |
| `global_while` | 1.1 |
| `struct_alias` | 1.3 |
| `struct_dup` | 1.2 |
| `struct_nested` | 1.3 |
