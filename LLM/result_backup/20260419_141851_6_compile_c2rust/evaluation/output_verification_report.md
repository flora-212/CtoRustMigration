# Output Verification Report

**Generated:** 2026-04-21T13:07:27.015414
**Total Examples:** 30
**Passed:** 21
**Failed:** 9

## Summary

- **Pass Rate:** 21/30 (70.0%)
- **Total Time:** 732.2s
- **Average Time:** 24.4s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Output verification passed | 4.6 |
| `array_main` | ✅ PASS | ✅ Output verification passed | 24.3 |
| `array_simple` | ✅ PASS | ✅ Output verification passed | 3.6 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 4.0 |
| `global_assume2` | ❌ FAIL | ❌ Output mismatch | 1.1 |
| `global_check` | ✅ PASS | ✅ Output verification passed | 2.9 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ✅ PASS | ✅ Output verification passed | 17.8 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 25.9 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 5.2 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 4.0 |
| `global_rwlock` | ✅ PASS | ✅ Output verification passed | 2.3 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 2.7 |
| `global_trylock` | ❌ FAIL | ❌ Output mismatch | 3.1 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 2.5 |
| `struct_alias` | ✅ PASS | ✅ Output verification passed | 2.7 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 3.2 |
| `struct_condvar` | ❌ FAIL | ❌ Output mismatch | 3.0 |
| `struct_dup` | ❌ FAIL | ❌ Output mismatch | 3.1 |
| `struct_empty` | ✅ PASS | ✅ Output verification passed | 6.4 |
| `struct_init` | ✅ PASS | ✅ Output verification passed | 2.9 |
| `struct_main` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_malloc` | ✅ PASS | ✅ Output verification passed | 0.8 |
| `struct_malloc2` | ✅ PASS | ✅ Output verification passed | 0.8 |
| `struct_multiple` | ❌ FAIL | ❌ Build failed | 0.0 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `struct_simple` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_spin` | ✅ PASS | ✅ Output verification passed | 1.2 |
| `struct_timedwait` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `unused_func` | ✅ PASS | ✅ Output verification passed | 1.0 |

## Failed Examples

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

### global_condvar

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

### global_trylock

```
Message: ❌ Output mismatch

Details:
Expected:
8

Actual:
0
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

### struct_main

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
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
   Compiling output_test v0.1.0 (/tmp/output_test_struct_multiple_7y1aquxt)
error[E0277]: `St` doesn't implement `Debug`
  --> src/main.rs:23:13
   |
23 |         })).unwrap();
   |             ^^^^^^ the trait `Debug` is not implemented for `St`
   |
   = note: add `#[derive(Debug)]` to `St` or manually `impl Debug for St`
h
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 4.6 |
| `array_main` | 24.3 |
| `array_simple` | 3.6 |
| `global_check` | 2.9 |
| `global_custom` | 17.8 |
| `global_nested` | 5.2 |
| `global_read` | 4.0 |
| `global_rwlock` | 2.3 |
| `global_simple` | 2.7 |
| `global_while` | 2.5 |
| `struct_alias` | 2.7 |
| `struct_assume` | 3.2 |
| `struct_empty` | 6.4 |
| `struct_init` | 2.9 |
| `struct_malloc` | 0.8 |
| `struct_malloc2` | 0.8 |
| `struct_nested` | 1.1 |
| `struct_simple` | 0.9 |
| `struct_spin` | 1.2 |
| `struct_timedwait` | 0.9 |
| `unused_func` | 1.0 |
