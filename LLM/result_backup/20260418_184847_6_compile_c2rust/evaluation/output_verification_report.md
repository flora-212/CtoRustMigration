# Output Verification Report

**Generated:** 2026-04-21T13:12:40.593537
**Total Examples:** 30
**Passed:** 20
**Failed:** 10

## Summary

- **Pass Rate:** 20/30 (66.7%)
- **Total Time:** 1563.9s
- **Average Time:** 52.1s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Output verification passed | 5.4 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 6.8 |
| `array_simple` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 5.8 |
| `global_assume2` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `global_check` | ✅ PASS | ✅ Output verification passed | 5.0 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 2.3 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 2.5 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 2.9 |
| `global_rwlock` | ✅ PASS | ✅ Output verification passed | 7.0 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 2.6 |
| `global_trylock` | ✅ PASS | ✅ Output verification passed | 4.9 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 2.1 |
| `struct_alias` | ✅ PASS | ✅ Output verification passed | 1.9 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 1.7 |
| `struct_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_dup` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_empty` | ✅ PASS | ✅ Output verification passed | 1.2 |
| `struct_init` | ✅ PASS | ✅ Output verification passed | 1.2 |
| `struct_main` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `struct_malloc` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_malloc2` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_multiple` | ❌ FAIL | ❌ Output mismatch | 1.2 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_simple` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `struct_spin` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `struct_timedwait` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `unused_func` | ✅ PASS | ✅ Output verification passed | 1.2 |

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

### struct_main

```
Message: ⏱️  Timeout (>300s)

Details:
Test execution exceeded timeout
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

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 5.4 |
| `array_simple` | 1.3 |
| `global_assume2` | 1.1 |
| `global_check` | 5.0 |
| `global_nested` | 2.5 |
| `global_read` | 2.9 |
| `global_rwlock` | 7.0 |
| `global_simple` | 2.6 |
| `global_trylock` | 4.9 |
| `global_while` | 2.1 |
| `struct_alias` | 1.9 |
| `struct_assume` | 1.7 |
| `struct_empty` | 1.2 |
| `struct_init` | 1.2 |
| `struct_malloc` | 0.9 |
| `struct_malloc2` | 1.3 |
| `struct_nested` | 0.9 |
| `struct_spin` | 1.0 |
| `struct_timedwait` | 0.9 |
| `unused_func` | 1.2 |
