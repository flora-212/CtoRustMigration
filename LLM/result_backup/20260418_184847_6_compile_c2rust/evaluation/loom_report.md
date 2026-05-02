# Loom Concurrency Test Report

**Generated:** 2026-04-21T12:47:43.003803
**Total Examples:** 62
**Passed:** 9
**Failed:** 53

## Summary

- **Pass Rate:** 9/62 (14.5%)
- **Total Time:** 912.7s
- **Average Time:** 14.7s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 10.1 |
| `array_const____deadlock` | ✅ PASS | ✅ Loom tests passed | 13.9 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Loom tests passed | 17.9 |
| `array_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 9.9 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 14.1 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 11.8 |
| `array_simple` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 14.0 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 17.4 |
| `global_assume` | ✅ PASS | ✅ Loom tests passed | 18.4 |
| `global_assume2` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 16.9 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 18.0 |
| `global_assume____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 17.0 |
| `global_check` | ❌ FAIL | ❌ error[E0425]: cannot find value `N` in this scope | 18.5 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 15.9 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 17.5 |
| `global_condvar` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1701733) panicked a | 18.4 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 17.0 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Loom tests passed | 16.8 |
| `global_custom` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 16.3 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 22.9 |
| `global_main` | ❌ FAIL | ❌ error: expected one of `!` or `::`, found `init` | 14.3 |
| `global_main____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 35.3 |
| `global_nested` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.3 |
| `global_nested____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 13.7 |
| `global_read` | ✅ PASS | ✅ Loom tests passed | 11.3 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 10.5 |
| `global_rwlock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 14.2 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 13.6 |
| `global_simple` | ✅ PASS | ✅ Loom tests passed | 11.6 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Loom tests passed | 11.0 |
| `global_trylock` | ✅ PASS | ✅ Loom tests passed | 12.4 |
| `global_while` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 15.5 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 13.5 |
| `struct_alias` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.3 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 14.9 |
| `struct_assume` | ❌ FAIL | ❌ error[E0133]: call to unsafe function `main::malloc` is un | 20.1 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0133]: call to unsafe function `main::malloc` is un | 16.2 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 15.9 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 4.0 |
| `struct_dup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1779253) panicked a | 14.6 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.6 |
| `struct_empty` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 12.9 |
| `struct_init` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 16.7 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 11.9 |
| `struct_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 14.4 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0133]: call to unsafe function `main::malloc` is un | 14.8 |
| `struct_malloc2` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 9.7 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `X` in this scope | 13.3 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0133]: call to unsafe function `main::malloc` is un | 13.0 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 11.7 |
| `struct_multiple____deadlock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 11.9 |
| `struct_nested` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 12.5 |
| `struct_nested____self_lock` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1813652) panicked a | 19.2 |
| `struct_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 12.7 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 17.0 |
| `struct_spin` | ❌ FAIL | ❌ error[E0133]: call to unsafe function `main::pthread_mutex | 12.2 |
| `struct_spin____lock_leak` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 18.3 |
| `struct_timedwait` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 17.4 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.1 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 11.3 |
| `unused_func` | ✅ PASS | ✅ Loom tests passed | 11.9 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 15.5 |

## Failures

### array_const

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling lazy_sta
```

### array_main

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   
```

### array_main____lock_leak

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   
```

### array_main____partial_critical_section

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   
```

### array_simple

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling lazy_sta
```

### array_simple____partial_critical_section

```
Message: ❌ error[E0425]: cannot find value `n1` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   
```

### global_assume2

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8
```

### global_assume2____self_lock

```
Message: ❌ error: expected one of `:`, `@`, or `|`, found `)`

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling pin-proj
```

### global_assume____lock_leak

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   
```

### global_check

```
Message: ❌ error[E0425]: cannot find value `N` in this scope

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   
```

### global_check____lock_leak

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling pin-proj
```

### global_check____lock_mismatch

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   
```

### global_condvar

```
Message: ❌ thread 'main::test_concurrent_access' (1701733) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling pin-proj
```

### global_condvar____lost_wakeup

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0
```

### global_custom

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   
```

### global_custom____self_lock

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0
```

### global_main

```
Message: ❌ error: expected one of `!` or `::`, found `init`

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8
```

### global_main____self_lock

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Co
```

### global_nested

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### global_nested____deadlock

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
```

### global_read____lock_mismatch

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling lazy_sta
```

### global_rwlock

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
```

### global_rwlock____lock_leak

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1
```

### global_while

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8
```

### global_while____lock_leak

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   
```

### struct_alias

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8
```

### struct_alias____self_lock

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   
```

### struct_assume

```
Message: ❌ error[E0133]: call to unsafe function `main::malloc` is unsafe and requires unsafe function or block

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4
```

### struct_assume____deadlock

```
Message: ❌ error[E0133]: call to unsafe function `main::malloc` is unsafe and requires unsafe function or block

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v
```

### struct_condvar

```
Message: ❌ error[E0425]: cannot find value `S` in this scope

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v
```

### struct_condvar____lost_wakeup

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_dup

```
Message: ❌ thread 'main::test_concurrent_access' (1779253) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
```

### struct_dup____deadlock

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   
```

### struct_empty

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling lazy_sta
```

### struct_init

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.
```

### struct_init____partial_critical_section

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Co
```

### struct_main

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   
```

### struct_malloc

```
Message: ❌ error[E0133]: call to unsafe function `main::malloc` is unsafe and requires unsafe function or block

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8
```

### struct_malloc2

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   
```

### struct_malloc2____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `X` in this scope

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling lazy_sta
```

### struct_malloc____lost_wakeup

```
Message: ❌ error[E0133]: call to unsafe function `main::malloc` is unsafe and requires unsafe function or block

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Co
```

### struct_multiple

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   
```

### struct_multiple____deadlock

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling pin-proj
```

### struct_nested

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8
```

### struct_nested____self_lock

```
Message: ❌ thread 'main::test_concurrent_access' (1813652) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   
```

### struct_simple

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
```

### struct_simple____partial_critical_section

```
Message: ❌ error[E0425]: cannot find value `S` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.
```

### struct_spin

```
Message: ❌ error[E0133]: call to unsafe function `main::pthread_mutex_init` is unsafe and requires unsafe funct

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8
```

### struct_spin____lock_leak

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0
```

### struct_timedwait

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   
```

### struct_timedwait____deadlock

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.
```

### unused_func____lock_mismatch

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const____deadlock` | 13.9 |
| `array_const____lock_mismatch` | 17.9 |
| `global_assume` | 18.4 |
| `global_condvar____partial_critical_section` | 16.8 |
| `global_read` | 11.3 |
| `global_simple` | 11.6 |
| `global_simple____partial_critical_section` | 11.0 |
| `global_trylock` | 12.4 |
| `unused_func` | 11.9 |
