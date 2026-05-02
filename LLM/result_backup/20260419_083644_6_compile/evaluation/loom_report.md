# Loom Concurrency Test Report

**Generated:** 2026-04-21T12:58:43.159110
**Total Examples:** 62
**Passed:** 7
**Failed:** 55

## Summary

- **Pass Rate:** 7/62 (11.3%)
- **Total Time:** 1470.8s
- **Average Time:** 23.7s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ error[E0277]: the trait bound `loom::sync::Mutex<i32>: Cop | 9.2 |
| `array_const____deadlock` | ✅ PASS | ✅ Loom tests passed | 18.3 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 16.8 |
| `array_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 16.0 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 17.8 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 17.2 |
| `array_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 16.4 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 17.0 |
| `global_assume` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 17.3 |
| `global_assume2` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 19.1 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 15.6 |
| `global_assume____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `N1` in this scope | 15.9 |
| `global_check` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 24.1 |
| `global_check____lock_leak` | ✅ PASS | ✅ Loom tests passed | 14.9 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `M` in this scope | 33.8 |
| `global_condvar` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1726012) panicked a | 14.6 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1729211) panicked a | 15.6 |
| `global_condvar____partial_critical_section` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1732699) panicked a | 10.4 |
| `global_custom` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1736135) panicked a | 10.1 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 11.8 |
| `global_main` | ✅ PASS | ✅ Loom tests passed | 13.6 |
| `global_main____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 11.3 |
| `global_nested` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 11.2 |
| `global_nested____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `handle2` in this scope | 12.4 |
| `global_read` | ✅ PASS | ✅ Loom tests passed | 15.9 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `handle2` in this scope | 14.1 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `LOCK` in this scope | 11.3 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `LOCK` in this scope | 15.0 |
| `global_simple` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `loo | 20.4 |
| `global_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 16.3 |
| `global_trylock` | ✅ PASS | ✅ Loom tests passed | 16.7 |
| `global_while` | ❌ FAIL | ⏱️  Timeout (>600s) | 600.0 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error[E0382]: use of moved value: `shared_data` | 20.3 |
| `struct_alias` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 8.2 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 10.1 |
| `struct_assume` | ❌ FAIL | ❌ error[E0502]: cannot borrow `*s` as mutable because it is  | 9.7 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for reference ` | 15.0 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 11.6 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 11.6 |
| `struct_dup` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 14.2 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 13.3 |
| `struct_empty` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 18.8 |
| `struct_init` | ❌ FAIL | ❌ error[E0308]: mismatched types | 9.2 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find type `Once` in this scope | 8.3 |
| `struct_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 8.1 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 7.0 |
| `struct_malloc2` | ❌ FAIL | ❌ error[E0425]: cannot find value `X` in this scope | 7.6 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `LOCK` in this scope | 30.8 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0308]: mismatched types | 10.7 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0425]: cannot find type `Lazy` in this scope | 28.6 |
| `struct_multiple____deadlock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 14.4 |
| `struct_nested` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 7.9 |
| `struct_nested____self_lock` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1929685) panicked a | 10.6 |
| `struct_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 10.2 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 12.8 |
| `struct_spin` | ✅ PASS | ✅ Loom tests passed | 34.8 |
| `struct_spin____lock_leak` | ❌ FAIL | ❌ error[E0308]: mismatched types | 7.9 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0599]: no method named `wait_until` found for struc | 8.9 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `SHARED_DATA` in this scop | 9.7 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 6.3 |
| `unused_func` | ✅ PASS | ✅ Loom tests passed | 7.8 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope | 5.9 |

## Failures

### array_const

```
Message: ❌ error[E0277]: the trait bound `loom::sync::Mutex<i32>: Copy` is not satisfied

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
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling pin-proj
```

### array_const____lock_mismatch

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
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
```

### array_main____lock_leak

```
Message: ❌ error[E0425]: cannot find value `num_mutex` in this scope

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

### array_main____partial_critical_section

```
Message: ❌ error[E0425]: cannot find value `num_mutex` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.
```

### array_simple

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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   
```

### global_assume

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling lazy_sta
```

### global_assume2

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
   Compiling libc v0.2.185
   
```

### global_assume2____self_lock

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
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
```

### global_assume____lock_leak

```
Message: ❌ error[E0425]: cannot find value `N1` in this scope

Details:
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
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   
```

### global_check

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
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
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling lazy_sta
```

### global_check____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `M` in this scope

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
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### global_condvar

```
Message: ❌ thread 'main::test_concurrent_access' (1726012) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
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
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
```

### global_condvar____lost_wakeup

```
Message: ❌ thread 'main::test_concurrent_access' (1729211) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
```

### global_condvar____partial_critical_section

```
Message: ❌ thread 'main::test_concurrent_access' (1732699) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling lazy_sta
```

### global_custom

```
Message: ❌ thread 'main::test_concurrent_access' (1736135) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   
```

### global_custom____self_lock

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-proj
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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8
```

### global_nested

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
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
   Compiling log v0.4.29
   Compiling libc v0.2.185
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
```

### global_read____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `handle2` in this scope

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
   Compiling log v0.4.29
   
```

### global_rwlock

```
Message: ❌ error[E0425]: cannot find value `LOCK` in this scope

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
   Compiling log v0.4.29
   Co
```

### global_rwlock____lock_leak

```
Message: ❌ error[E0425]: cannot find value `LOCK` in this scope

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

### global_simple

```
Message: ❌ error[E0599]: no method named `lock` found for struct `loom::sync::Arc<SharedData>` in the current s

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
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling pin-proj
```

### global_simple____partial_critical_section

```
Message: ❌ error[E0308]: mismatched types

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

### global_while

```
Message: ⏱️  Timeout (>600s)

Details:
Loom state space exploration exceeded timeout
```

### global_while____lock_leak

```
Message: ❌ error[E0382]: use of moved value: `shared_data`

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

### struct_alias

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

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
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling lazy_sta
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
   Compiling regex-syntax v0.8
```

### struct_assume

```
Message: ❌ error[E0502]: cannot borrow `*s` as mutable because it is also borrowed as immutable

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
   Compiling log v0.4.29
   Compiling libc v0.2.185
   
```

### struct_assume____deadlock

```
Message: ❌ error[E0599]: no method named `lock` found for reference `&loom::sync::Arc<Ss>` in the current scope

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-proj
```

### struct_condvar

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   
```

### struct_condvar____lost_wakeup

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### struct_dup

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
   Compiling cfg-if v1.0.4
   
```

### struct_dup____deadlock

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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   
```

### struct_empty

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
   Compiling log v0.4.29
   Co
```

### struct_init

```
Message: ❌ error[E0308]: mismatched types

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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
```

### struct_init____partial_critical_section

```
Message: ❌ error[E0425]: cannot find type `Once` in this scope

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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.
```

### struct_malloc

```
Message: ❌ error[E0308]: mismatched types

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
   Compiling libc v0.2.
```

### struct_malloc2

```
Message: ❌ error[E0425]: cannot find value `X` in this scope

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
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8
```

### struct_malloc2____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `LOCK` in this scope

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
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling pin-proj
```

### struct_malloc____lost_wakeup

```
Message: ❌ error[E0308]: mismatched types

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

### struct_multiple

```
Message: ❌ error[E0425]: cannot find type `Lazy` in this scope

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

### struct_multiple____deadlock

```
Message: ❌ error: test failed, to rerun pass `--lib`

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

### struct_nested

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling lazy_sta
```

### struct_nested____self_lock

```
Message: ❌ thread 'main::test_concurrent_access' (1929685) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   
```

### struct_spin____lock_leak

```
Message: ❌ error[E0308]: mismatched types

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

### struct_timedwait

```
Message: ❌ error[E0599]: no method named `wait_until` found for struct `loom::sync::Condvar` in the current sco

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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
```

### struct_timedwait____deadlock

```
Message: ❌ error[E0425]: cannot find value `SHARED_DATA` in this scope

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

### struct_timedwait____lost_wakeup

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
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0
```

### unused_func____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling lazy_sta
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const____deadlock` | 18.3 |
| `global_check____lock_leak` | 14.9 |
| `global_main` | 13.6 |
| `global_read` | 15.9 |
| `global_trylock` | 16.7 |
| `struct_spin` | 34.8 |
| `unused_func` | 7.8 |
