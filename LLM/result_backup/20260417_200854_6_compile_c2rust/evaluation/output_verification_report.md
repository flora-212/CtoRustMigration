# Output Verification Report

**Generated:** 2026-04-21T13:04:49.241609
**Total Examples:** 30
**Passed:** 21
**Failed:** 9

## Summary

- **Pass Rate:** 21/30 (70.0%)
- **Total Time:** 471.7s
- **Average Time:** 15.7s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Output verification passed | 6.8 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 3.4 |
| `array_simple` | ✅ PASS | ✅ Output verification passed | 4.2 |
| `global_assume` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `global_assume2` | ✅ PASS | ✅ Output verification passed | 4.6 |
| `global_check` | ✅ PASS | ✅ Output verification passed | 4.3 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ❌ Output mismatch | 10.1 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 2.6 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 4.6 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 3.8 |
| `global_rwlock` | ✅ PASS | ✅ Output verification passed | 19.3 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 3.0 |
| `global_trylock` | ❌ FAIL | ❌ Output mismatch | 4.4 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 6.4 |
| `struct_alias` | ✅ PASS | ✅ Output verification passed | 20.9 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 25.9 |
| `struct_condvar` | ✅ PASS | ✅ Output verification passed | 5.4 |
| `struct_dup` | ❌ FAIL | ❌ Output mismatch | 3.8 |
| `struct_empty` | ✅ PASS | ✅ Output verification passed | 2.6 |
| `struct_init` | ✅ PASS | ✅ Output verification passed | 2.6 |
| `struct_main` | ❌ FAIL | ❌ Output mismatch | 3.0 |
| `struct_malloc` | ✅ PASS | ✅ Output verification passed | 2.4 |
| `struct_malloc2` | ✅ PASS | ✅ Output verification passed | 2.7 |
| `struct_multiple` | ❌ FAIL | ❌ Output mismatch | 3.2 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 2.8 |
| `struct_simple` | ❌ FAIL | ❌ Output mismatch | 3.0 |
| `struct_spin` | ✅ PASS | ✅ Output verification passed | 6.8 |
| `struct_timedwait` | ✅ PASS | ✅ Output verification passed | 2.6 |
| `unused_func` | ✅ PASS | ✅ Output verification passed | 5.1 |

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

### global_trylock

```
Message: ❌ Output mismatch

Details:
Expected:
8

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
Message: ❌ Output mismatch

Details:
Expected:
4

Actual:
0
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
| `array_const` | 6.8 |
| `array_simple` | 4.2 |
| `global_assume` | 1.1 |
| `global_assume2` | 4.6 |
| `global_check` | 4.3 |
| `global_nested` | 4.6 |
| `global_read` | 3.8 |
| `global_rwlock` | 19.3 |
| `global_simple` | 3.0 |
| `global_while` | 6.4 |
| `struct_alias` | 20.9 |
| `struct_assume` | 25.9 |
| `struct_condvar` | 5.4 |
| `struct_empty` | 2.6 |
| `struct_init` | 2.6 |
| `struct_malloc` | 2.4 |
| `struct_malloc2` | 2.7 |
| `struct_nested` | 2.8 |
| `struct_spin` | 6.8 |
| `struct_timedwait` | 2.6 |
| `unused_func` | 5.1 |
