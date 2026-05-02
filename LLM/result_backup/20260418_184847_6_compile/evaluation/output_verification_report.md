# Output Verification Report

**Generated:** 2026-04-21T13:11:46.774794
**Total Examples:** 30
**Passed:** 8
**Failed:** 22

## Summary

- **Pass Rate:** 8/30 (26.7%)
- **Total Time:** 1572.1s
- **Average Time:** 52.4s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ Build failed | 0.0 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 13.9 |
| `array_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 9.5 |
| `global_assume2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_check` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 10.2 |
| `global_nested` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 4.5 |
| `global_rwlock` | ✅ PASS | ✅ Output verification passed | 3.6 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 19.3 |
| `global_trylock` | ✅ PASS | ✅ Output verification passed | 2.8 |
| `global_while` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_alias` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_assume` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_dup` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_empty` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `struct_init` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_main` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_malloc` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_malloc2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_multiple` | ❌ FAIL | ❌ Output mismatch | 2.1 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_simple` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `struct_spin` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_timedwait` | ❌ FAIL | ❌ Build failed | 0.0 |
| `unused_func` | ✅ PASS | ✅ Output verification passed | 2.1 |

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
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_array_const_j9imq
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
   Compiling output_test v0.1.0 (/tmp/output_test_array_simple_mn2w
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
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_assume2_6q
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
2
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
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_nested_tay4uw98)
warning: variable does not need to be mutable
```

### global_while

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
   Compiling output_test v0.1.0 (/tmp/output_test_global_while_6m395z5u)
error[E0425]: cannot find function `pthread_mu
```

### struct_alias

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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_alias_4nrn10w3)
error[E0502]: cannot borrow `*s` as mutable because it is also borrowed as immutable
  --> src/mai
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
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_assume_zi5v3pe5)
error[E0308]: mismatched types
  --> s
```

### struct_condvar

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### struct_dup

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_init_kf9n1uk1)
error[E0658]: use of unstable library feature `get_mut_unchecked`
  --> src/main.rs:38:8
   |
38 |     f1(Arc::get_mut_unchecked(s1) as *mut Ss);
   |        ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #63292 <https://github.com/rust-lang/rust/issues
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
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc_sxht3fad)
error[E0308]: mismatched types
  --> src/main.rs:16:18
   |
16 |         s.c.wait(&mut s);
   |             ---- ^^^^^^ expected `MutexGuard<'_, _>`, found `&mut MutexGuard<'_, S>`
   |             |
   |             arguments to this method are incorre
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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc2_nbgqhfce)
warning: variable does not need to be mutable
  --> src/main.rs:25:9
   |
25 |     let mut guard = s.m.lock().unwrap();
   |         ----^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(un
```

### struct_multiple

```
Message: ❌ Output mismatch

Details:
Expected:
2 3 4

Actual:
0 1 2
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
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_spin_oj3mln44)
error[E0308]: mismatched types
  --> src/main.rs:46:34
   |
46 |         let s_clone = Arc::clone(&s);
   |                       ---------- ^^ expected `&Arc<_, _>`, found `&Ss`
   |                       |
   |                       arguments to this fu
```

### struct_timedwait

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_timedwait_e4_0oyll)
warning: unnecessary parentheses around function argument
  --> src/main.rs:35:78
   |
35 |         let (guard, _) = state.cond.wait_timeout(guard, Duration::from_nanos((ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64))).unwrap();
   |          
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `global_check` | 1.3 |
| `global_read` | 4.5 |
| `global_rwlock` | 3.6 |
| `global_simple` | 19.3 |
| `global_trylock` | 2.8 |
| `struct_empty` | 1.0 |
| `struct_nested` | 0.9 |
| `unused_func` | 2.1 |
