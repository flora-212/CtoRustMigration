# Loom Concurrency Test Report

**Generated:** 2026-04-21T12:48:27.887144
**Total Examples:** 62
**Passed:** 9
**Failed:** 53

## Summary

- **Pass Rate:** 9/62 (14.5%)
- **Total Time:** 1230.1s
- **Average Time:** 19.8s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Loom tests passed | 7.9 |
| `array_const____deadlock` | ❌ FAIL | ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't im | 5.3 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 6.0 |
| `array_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 6.4 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 4.9 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 6.2 |
| `array_simple` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 5.6 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 6.1 |
| `global_assume` | ✅ PASS | ✅ Loom tests passed | 8.4 |
| `global_assume2` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 7.6 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 6.2 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Loom tests passed | 6.9 |
| `global_check` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 5.2 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 6.6 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 4.4 |
| `global_condvar` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1608156) panicked a | 7.8 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 5.7 |
| `global_condvar____partial_critical_section` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1610855) panicked a | 7.9 |
| `global_custom` | ❌ FAIL | ❌ error[E0428]: the name `lock` is defined multiple times | 9.0 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't im | 9.7 |
| `global_main` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 6.4 |
| `global_main____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 8.2 |
| `global_nested` | ✅ PASS | ✅ Loom tests passed | 6.5 |
| `global_nested____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `handle2` in this scope | 7.5 |
| `global_read` | ✅ PASS | ✅ Loom tests passed | 7.1 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope | 7.3 |
| `global_rwlock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 6.5 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0428]: the name `f1` is defined multiple times | 6.8 |
| `global_simple` | ✅ PASS | ✅ Loom tests passed | 8.3 |
| `global_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 9.9 |
| `global_trylock` | ✅ PASS | ✅ Loom tests passed | 10.4 |
| `global_while` | ❌ FAIL | ⏱️  Timeout (>600s) | 600.0 |
| `global_while____lock_leak` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1767638) panicked a | 14.0 |
| `struct_alias` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 15.5 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 15.9 |
| `struct_assume` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 17.1 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1782148) panicked a | 15.6 |
| `struct_condvar` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 10.8 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 17.4 |
| `struct_dup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 11.4 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 13.9 |
| `struct_empty` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 14.9 |
| `struct_init` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 10.6 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.9 |
| `struct_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 12.5 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0133]: call to unsafe function `main::malloc` is un | 13.2 |
| `struct_malloc2` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 10.0 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `X` in this scope | 13.3 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1813633) panicked a | 18.3 |
| `struct_multiple` | ❌ FAIL | ❌ error: visibility `pub` is not followed by an item | 12.4 |
| `struct_multiple____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 4.3 |
| `struct_nested` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 13.7 |
| `struct_nested____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 12.3 |
| `struct_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 17.9 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 19.0 |
| `struct_spin` | ❌ FAIL | ❌ error[E0133]: call to unsafe function `main::pthread_mutex | 10.4 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Loom tests passed | 11.9 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0425]: cannot find value `SHARED_STATE` in this sco | 11.7 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 16.1 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 17.5 |
| `unused_func` | ✅ PASS | ✅ Loom tests passed | 14.3 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1852006) panicked a | 12.6 |

## Failures

### array_const____deadlock

```
Message: ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't implement `std::fmt::Display`

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

### array_const____lock_mismatch

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
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.
```

### array_main

```
Message: ❌ error[E0425]: cannot find value `num_mutex` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling pin-proj
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
```

### array_main____partial_critical_section

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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0
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
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling lazy_sta
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.
```

### global_assume2____self_lock

```
Message: ❌ error: expected one of `:`, `@`, or `|`, found `)`

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
   Compiling libc v0.2.
```

### global_check

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8
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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
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
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
```

### global_condvar

```
Message: ❌ thread 'main::test_concurrent_access' (1608156) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling pin-proj
```

### global_condvar____partial_critical_section

```
Message: ❌ thread 'main::test_concurrent_access' (1610855) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling lazy_sta
```

### global_custom

```
Message: ❌ error[E0428]: the name `lock` is defined multiple times

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
   Compiling log v0.4.29
   Compiling cfg-if v1.
```

### global_custom____self_lock

```
Message: ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't implement `std::fmt::Display`

Details:
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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
```

### global_main

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
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling pin-proj
```

### global_nested____deadlock

```
Message: ❌ error[E0425]: cannot find value `handle2` in this scope

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
   Compiling regex-syntax v0.8
```

### global_read____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling lazy_sta
```

### global_rwlock

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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### global_rwlock____lock_leak

```
Message: ❌ error[E0428]: the name `f1` is defined multiple times

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling pin-proj
```

### global_simple____partial_critical_section

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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   
```

### global_while

```
Message: ⏱️  Timeout (>600s)

Details:
Loom state space exploration exceeded timeout
```

### global_while____lock_leak

```
Message: ❌ thread 'main::test_concurrent_access' (1767638) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling lazy_sta
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Co
```

### struct_assume

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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8
```

### struct_assume____deadlock

```
Message: ❌ thread 'main::test_concurrent_access' (1782148) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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

### struct_condvar

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
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0
```

### struct_condvar____lost_wakeup

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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.
```

### struct_dup

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-synt
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
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v
```

### struct_empty

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
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   
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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   
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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling libc v0.2.185
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4
```

### struct_malloc____lost_wakeup

```
Message: ❌ thread 'main::test_concurrent_access' (1813633) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
Message: ❌ error: visibility `pub` is not followed by an item

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
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   
```

### struct_multiple____deadlock

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_nested

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
   Compiling log v0.4.29
   
```

### struct_nested____self_lock

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
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
```

### struct_simple

```
Message: ❌ error[E0425]: cannot find value `S` in this scope

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
   Compiling regex-syntax v0.8
```

### struct_simple____partial_critical_section

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
```

### struct_timedwait

```
Message: ❌ error[E0425]: cannot find value `SHARED_STATE` in this scope

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

### struct_timedwait____deadlock

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   
```

### unused_func____lock_mismatch

```
Message: ❌ thread 'main::test_concurrent_access' (1852006) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 7.9 |
| `global_assume` | 8.4 |
| `global_assume____lock_leak` | 6.9 |
| `global_nested` | 6.5 |
| `global_read` | 7.1 |
| `global_simple` | 8.3 |
| `global_trylock` | 10.4 |
| `struct_spin____lock_leak` | 11.9 |
| `unused_func` | 14.3 |
