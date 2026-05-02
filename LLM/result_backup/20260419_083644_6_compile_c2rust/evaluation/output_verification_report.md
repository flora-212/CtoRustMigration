# Output Verification Report

**Generated:** 2026-04-21T13:10:35.533611
**Total Examples:** 30
**Passed:** 19
**Failed:** 11

## Summary

- **Pass Rate:** 19/30 (63.3%)
- **Total Time:** 633.4s
- **Average Time:** 21.1s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Output verification passed | 1.6 |
| `array_main` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `array_simple` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `global_assume` | ❌ FAIL | ❌ Output mismatch | 1.1 |
| `global_assume2` | ✅ PASS | ✅ Output verification passed | 1.2 |
| `global_check` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `global_condvar` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_custom` | ❌ FAIL | ⏱️  Timeout (>300s) | 300.0 |
| `global_main` | ❌ FAIL | ❌ Output mismatch | 1.1 |
| `global_nested` | ✅ PASS | ✅ Output verification passed | 1.4 |
| `global_read` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `global_rwlock` | ✅ PASS | ✅ Output verification passed | 1.2 |
| `global_simple` | ✅ PASS | ✅ Output verification passed | 1.0 |
| `global_trylock` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `global_while` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_alias` | ✅ PASS | ✅ Output verification passed | 1.4 |
| `struct_assume` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_condvar` | ✅ PASS | ✅ Output verification passed | 0.9 |
| `struct_dup` | ✅ PASS | ✅ Output verification passed | 1.8 |
| `struct_empty` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `struct_init` | ✅ PASS | ✅ Output verification passed | 1.5 |
| `struct_main` | ❌ FAIL | ❌ Output mismatch | 1.3 |
| `struct_malloc` | ✅ PASS | ✅ Output verification passed | 1.1 |
| `struct_malloc2` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_multiple` | ✅ PASS | ✅ Output verification passed | 1.5 |
| `struct_nested` | ✅ PASS | ✅ Output verification passed | 1.3 |
| `struct_simple` | ❌ FAIL | ❌ Output mismatch | 1.0 |
| `struct_spin` | ❌ FAIL | ❌ Output mismatch | 0.9 |
| `struct_timedwait` | ✅ PASS | ✅ Output verification passed | 1.2 |
| `unused_func` | ❌ FAIL | ❌ Output mismatch | 1.2 |

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
0
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

### struct_empty

```
Message: ❌ Output mismatch

Details:
Expected:
0 0

Actual:

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
| `array_const` | 1.6 |
| `array_simple` | 0.9 |
| `global_assume2` | 1.2 |
| `global_check` | 1.1 |
| `global_nested` | 1.4 |
| `global_read` | 1.1 |
| `global_rwlock` | 1.2 |
| `global_simple` | 1.0 |
| `global_while` | 0.9 |
| `struct_alias` | 1.4 |
| `struct_assume` | 1.3 |
| `struct_condvar` | 0.9 |
| `struct_dup` | 1.8 |
| `struct_init` | 1.5 |
| `struct_malloc` | 1.1 |
| `struct_malloc2` | 1.3 |
| `struct_multiple` | 1.5 |
| `struct_nested` | 1.3 |
| `struct_timedwait` | 1.2 |
