# Output Verification Report

**Generated:** 2026-04-21T12:10:02.544864
**Total Examples:** 30
**Passed:** 9
**Failed:** 21

## Summary

- **Pass Rate:** 9/30 (30.0%)
- **Total Time:** 315.9s
- **Average Time:** 10.5s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 1.1 |
| `array_simple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_assume` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `global_assume2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_check` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 0.8 |
| `global_rwlock` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `global_trylock` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_alias` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_condvar` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_dup` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `struct_empty` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_init` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_main` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `struct_malloc` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_malloc2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_multiple` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `struct_nested` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_simple` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `struct_spin` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_timedwait` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `unused_func` | ✅ PASS | ✅ Output verification passed | 0.9 |

## Failed Examples

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
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_array_simple_sepyhdk8)
error[E0308]: mismatched types
   --> src/main.rs:59:13
    |
 58 |         libc::printf(
    |         ------------ arguments to this function are incorrect
 59 |             b"%d %d %d %d %d\n\0".as_ptr(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

### global_assume2

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_global_assume2_jvme25nh)
error[E0425]: cannot find function `pthread_mutex_lock` in this scope
  --> src/main.rs:90:5
   |
90 |     pthread_mutex_lock(&raw mut num_mutex);
   |     ^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
 1 +
```

### global_check

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_check_jn4b6q86)
error[E0425]: cannot find function `pthread_mutex_lock` in this scope
  --> src/main.rs:69:5
   |
69 |     pthread_mutex_lock(&mut m);
   |     ^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
 1 + use libc::pth
```

### global_condvar

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

### global_custom

```
Message: ❌ Output mismatch

Details:
Expected:
8

Actual:
6
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

### global_rwlock

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_global_rwlock_358aggh8)
warning: unused import: `Arc`
 --> src/main.rs:1:17
  |
1 | use std::sync::{Arc, Mutex};
  |                 ^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::ffi::CStr`
 --> src/main.rs
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

### struct_alias

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_alias_pmgahlz1)
error[E0015]: cannot call non-const associated function `Arc::<std::sync::Mutex<St>>::new` in statics
  --> src/main.rs:16:33
   |
16 |   pub static S1: Arc<Mutex<St>> = Arc::new(M...
   |  _________________________________^
17 | |     n1: 0,
18 | |     
```

### struct_condvar

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_condvar_3hpl4mnm)
error[E0594]: cannot assign to data in an `Arc`
  --> src/main.rs:33:5
   |
33 |     s.n1 += 1;
   |     ^^^^^^^^^ cannot assign
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<SharedData
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
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling output_test v0.1.0 (/tmp/output_test_struct_empty_czbkter7)
error[E0015]: cannot call non-const associated function `Arc::<std::sync::Mutex<Ss>>::new` in statics
  --> src/main.rs:12:29
   |
12 |   static S1: Arc<Mutex<Ss>> = Arc::new(Mutex...
   |  _____________________________^
13 | |     n: 0,
14 | |     m: Mu
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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_init_c5qxfj3g)
error[E0614]: type `Ss` cannot be dereferenced
  --> src/main.rs:29:19
   |
29 | ...1 = &mut **S1.lock().unwrap() as *mut Ss;
   |             ^^^^^^^^^^^^^^^^^^^^ can't be dereferenced

error[E0614]: type `Ss` cannot be dereferenced
  --> src/main.rs:30:
```

### struct_main

```
Message: ❌ Output mismatch

Details:
Expected:
4

Actual:
0
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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc_33kakd1n)
error[E0308]: mismatched types
  --> src/main.rs:18:22
   |
18 |             s.c.wait(&mut guard);
   |                 ---- ^^^^^^^^^^ expected `MutexGuard<'_, _>`, found `&mut MutexGuard<'_, ()>`
   |                 |
   |                 arguments t
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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_malloc2_q_246c84)
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

### struct_nested

```
Message: ❌ Build failed

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling output_test v0.1.0 (/tmp/output_test_struct_nested_rxul8sjt)
warning: unused import: `std::ffi::CString`
 --> src/main.rs:3:5
  |
3 | use std::ffi::CString;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error[E0308]: mismatched types
   --> src/main.r
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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_spin_vseen08x)
error[E0308]: mismatched types
  --> src/main.rs:40:31
   |
40 |     let s_clone1 = Arc::clone(&s);
   |                    ---------- ^^ expected `&Arc<_, _>`, found `&Ss`
   |                    |
   |                    arguments to this function are i
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 1.1 |
| `global_assume` | 1.1 |
| `global_nested` | 0.9 |
| `global_read` | 0.8 |
| `global_simple` | 0.9 |
| `global_while` | 0.9 |
| `struct_assume` | 0.9 |
| `struct_timedwait` | 0.9 |
| `unused_func` | 0.9 |
