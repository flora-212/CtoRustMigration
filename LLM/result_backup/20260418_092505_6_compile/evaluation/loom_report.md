# Loom Concurrency Test Report

**Generated:** 2026-04-21T12:51:53.698795
**Total Examples:** 62
**Passed:** 8
**Failed:** 54

## Summary

- **Pass Rate:** 8/62 (12.9%)
- **Total Time:** 1316.3s
- **Average Time:** 21.2s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 7.8 |
| `array_const____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `loo | 6.7 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `loo | 6.8 |
| `array_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 7.9 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 6.3 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 6.5 |
| `array_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 6.6 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 8.8 |
| `global_assume` | ❌ FAIL | ❌ error: expected one of `!` or `::`, found `init` | 6.5 |
| `global_assume2` | ❌ FAIL | ❌ error[E0425]: cannot find value `N1` in this scope | 7.0 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 8.6 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Loom tests passed | 9.6 |
| `global_check` | ❌ FAIL | ❌ error[E0596]: cannot borrow data in dereference of `loom:: | 10.1 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 6.7 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `M` in this scope | 7.8 |
| `global_condvar` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1641615) panicked a | 8.6 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1643640) panicked a | 8.9 |
| `global_condvar____partial_critical_section` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1645676) panicked a | 5.7 |
| `global_custom` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 6.9 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0446]: private type `SharedData` in public interfac | 8.6 |
| `global_main` | ✅ PASS | ✅ Loom tests passed | 10.6 |
| `global_main____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 12.4 |
| `global_nested` | ✅ PASS | ✅ Loom tests passed | 17.4 |
| `global_nested____deadlock` | ❌ FAIL | ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't im | 11.6 |
| `global_read` | ✅ PASS | ✅ Loom tests passed | 14.4 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.1 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `LOCK` in this scope | 16.3 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 15.4 |
| `global_simple` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 16.8 |
| `global_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 17.6 |
| `global_trylock` | ✅ PASS | ✅ Loom tests passed | 18.2 |
| `global_while` | ❌ FAIL | ⏱️  Timeout (>600s) | 600.0 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 5.7 |
| `struct_alias` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 13.1 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1817219) panicked a | 19.5 |
| `struct_assume` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 14.0 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for reference ` | 13.1 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0432]: unresolved import `loom::sync::ONCE_INIT` | 16.8 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1829400) panicked a | 20.1 |
| `struct_dup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 9.5 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 11.1 |
| `struct_empty` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 12.1 |
| `struct_init` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 16.4 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 17.1 |
| `struct_main` | ✅ PASS | ✅ Loom tests passed | 14.9 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in dereference of `loo | 11.6 |
| `struct_malloc2` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 10.1 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `X` in this scope | 13.0 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in dereference of `loo | 15.4 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 23.6 |
| `struct_multiple____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `id1` in this scope | 13.5 |
| `struct_nested` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.1 |
| `struct_nested____self_lock` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1868608) panicked a | 5.7 |
| `struct_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 4.7 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 8.0 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 9.0 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Loom tests passed | 7.8 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0425]: cannot find value `SHARED_STATE` in this sco | 10.0 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 28.7 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 19.1 |
| `unused_func` | ✅ PASS | ✅ Loom tests passed | 17.9 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope | 7.4 |

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
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   
```

### array_const____deadlock

```
Message: ❌ error[E0599]: no method named `lock` found for struct `loom::sync::Arc<MutexArray>` in the current s

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

### array_const____lock_mismatch

```
Message: ❌ error[E0599]: no method named `lock` found for struct `loom::sync::Arc<MutexArray>` in the current s

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

### array_main

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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling lazy_sta
```

### array_main____lock_leak

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Co
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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### array_simple

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
```

### global_assume

```
Message: ❌ error: expected one of `!` or `::`, found `init`

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

### global_assume2

```
Message: ❌ error[E0425]: cannot find value `N1` in this scope

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
   Compiling log v0.4.29
   
```

### global_assume2____self_lock

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
   Compiling regex-syntax v0.8
```

### global_check

```
Message: ❌ error[E0596]: cannot borrow data in dereference of `loom::sync::Arc<MutexData>` as mutable

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
   Compiling cfg-if v1.0.4
   Compiling libc v0.
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
```

### global_condvar

```
Message: ❌ thread 'main::test_concurrent_access' (1641615) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling cfg-if v1.0.4
   
```

### global_condvar____lost_wakeup

```
Message: ❌ thread 'main::test_concurrent_access' (1643640) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling log v0.4.29
   Co
```

### global_condvar____partial_critical_section

```
Message: ❌ thread 'main::test_concurrent_access' (1645676) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_custom

```
Message: ❌ error: expected one of `:`, `@`, or `|`, found `)`

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

### global_custom____self_lock

```
Message: ❌ error[E0446]: private type `SharedData` in public interface

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

### global_nested____deadlock

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### global_read____lock_mismatch

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
   Compiling regex-syntax v0.8
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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   
```

### global_simple

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

### global_simple____partial_critical_section

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 30 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   
```

### global_while

```
Message: ⏱️  Timeout (>600s)

Details:
Loom state space exploration exceeded timeout
```

### global_while____lock_leak

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8
```

### struct_alias____self_lock

```
Message: ❌ thread 'main::test_concurrent_access' (1817219) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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

### struct_assume

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
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.
```

### struct_assume____deadlock

```
Message: ❌ error[E0599]: no method named `lock` found for reference `&loom::sync::Arc<Ss>` in the current scope

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
   Compiling regex-syntax v0
```

### struct_condvar

```
Message: ❌ error[E0432]: unresolved import `loom::sync::ONCE_INIT`

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
   Compiling log v0.4
```

### struct_condvar____lost_wakeup

```
Message: ❌ thread 'main::test_concurrent_access' (1829400) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-proj
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling pin-proj
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
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   
```

### struct_empty

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4
```

### struct_init

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
   Compiling log v0.4.29
   Co
```

### struct_init____partial_critical_section

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

### struct_malloc

```
Message: ❌ error[E0594]: cannot assign to data in dereference of `loom::sync::Arc<S>`

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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling lazy_sta
```

### struct_malloc____lost_wakeup

```
Message: ❌ error[E0594]: cannot assign to data in dereference of `loom::sync::Arc<S>`

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
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8
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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   
```

### struct_multiple____deadlock

```
Message: ❌ error[E0425]: cannot find value `id1` in this scope

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
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling pin-proj
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   
```

### struct_nested____self_lock

```
Message: ❌ thread 'main::test_concurrent_access' (1868608) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_simple

```
Message: ❌ error[E0425]: cannot find value `S` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
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

### struct_spin

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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   
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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.
```

### struct_timedwait____deadlock

```
Message: ❌ error[E0255]: the name `timespec` is defined multiple times

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
   Compiling regex-syntax v0.8
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0255]: the name `timespec` is defined multiple times

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

### unused_func____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope

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

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `global_assume____lock_leak` | 9.6 |
| `global_main` | 10.6 |
| `global_nested` | 17.4 |
| `global_read` | 14.4 |
| `global_trylock` | 18.2 |
| `struct_main` | 14.9 |
| `struct_spin____lock_leak` | 7.8 |
| `unused_func` | 17.9 |
