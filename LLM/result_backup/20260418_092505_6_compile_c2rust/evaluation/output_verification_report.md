# Output Verification Report

**Generated:** 2026-04-21T13:14:31.997572
**Total Examples:** 30
**Passed:** 20
**Failed:** 10

## Summary

- **Pass Rate:** 20/30 (66.7%)
- **Total Time:** 1278.2s
- **Average Time:** 42.6s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Output verification passed | 18.2 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 3.0 |
| `array_simple` | ✅ PASS | ✅ Output verification passed | 4.1 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 6.7 |
| `global_assume2` | ❌ FAIL | ❌ Build failed | 0.0 |
| `global_check` | ✅ PASS | ✅ Output verification passed | 26.0 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 1.2 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 1.5 |
| `global_rwlock` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `global_trylock` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `struct_alias` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_dup` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `struct_empty` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_init` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_main` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `struct_malloc` | ✅ PASS | ✅ Output verification passed | 0.8 |
| `struct_malloc2` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `struct_multiple` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_simple` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_spin` | ✅ PASS | ✅ Output verification passed | 1.1 |
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
   Compiling output_test v0.1.0 (/tmp/output_test_global_assume2_ks9q1cs4)
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
3
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

### struct_main

```
Message: ❌ Output mismatch

Details:
Expected:
4

Actual:
3
```

### struct_multiple

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 18.2 |
| `array_simple` | 4.1 |
| `global_check` | 26.0 |
| `global_nested` | 0.9 |
| `global_read` | 1.5 |
| `global_rwlock` | 0.9 |
| `global_simple` | 1.0 |
| `global_trylock` | 1.0 |
| `global_while` | 1.0 |
| `struct_alias` | 1.0 |
| `struct_assume` | 0.9 |
| `struct_empty` | 0.9 |
| `struct_init` | 0.9 |
| `struct_malloc` | 0.8 |
| `struct_malloc2` | 1.1 |
| `struct_nested` | 1.3 |
| `struct_simple` | 0.9 |
| `struct_spin` | 1.1 |
| `struct_timedwait` | 0.9 |
| `unused_func` | 0.9 |
