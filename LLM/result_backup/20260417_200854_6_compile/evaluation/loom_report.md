# Loom Concurrency Test Report

**Generated:** 2026-04-21T12:04:59.570838
**Total Examples:** 62
**Passed:** 9
**Failed:** 53

## Summary

- **Pass Rate:** 9/62 (14.5%)
- **Total Time:** 838.7s
- **Average Time:** 13.5s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Loom tests passed | 3.8 |
| `array_const____deadlock` | ❌ FAIL | ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't im | 3.8 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 3.4 |
| `array_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 3.7 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 3.7 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 3.7 |
| `array_simple` | ❌ FAIL | ❌ error[E0428]: the name `f1` is defined multiple times | 3.7 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 3.3 |
| `global_assume` | ✅ PASS | ✅ Loom tests passed | 3.5 |
| `global_assume2` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 3.2 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 3.4 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Loom tests passed | 3.5 |
| `global_check` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 3.2 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 3.2 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 3.4 |
| `global_condvar` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1481731) panicked a | 5.2 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `N1` in this scope | 3.4 |
| `global_condvar____partial_critical_section` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1482758) panicked a | 4.4 |
| `global_custom` | ❌ FAIL | ❌ error[E0428]: the name `lock` is defined multiple times | 3.4 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't im | 3.4 |
| `global_main` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 3.1 |
| `global_main____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 3.6 |
| `global_nested` | ✅ PASS | ✅ Loom tests passed | 3.4 |
| `global_nested____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `handle2` in this scope | 3.3 |
| `global_read` | ✅ PASS | ✅ Loom tests passed | 3.6 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope | 3.4 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `LOCK` in this scope | 3.5 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0428]: the name `f1` is defined multiple times | 3.2 |
| `global_simple` | ✅ PASS | ✅ Loom tests passed | 3.3 |
| `global_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 3.9 |
| `global_trylock` | ✅ PASS | ✅ Loom tests passed | 3.5 |
| `global_while` | ❌ FAIL | ⏱️  Timeout (>600s) | 600.0 |
| `global_while____lock_leak` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1493971) panicked a | 5.7 |
| `struct_alias` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 9.6 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 4.1 |
| `struct_assume` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 4.7 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1496199) panicked a | 6.2 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 4.7 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 3.6 |
| `struct_dup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 5.6 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 3.5 |
| `struct_empty` | ❌ FAIL | ❌ error[E0596]: cannot borrow `s1` as mutable, as it is not  | 3.9 |
| `struct_init` | ❌ FAIL | ❌ error[E0614]: type `Ss` cannot be dereferenced | 3.5 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error: cast cannot be followed by a method call | 3.3 |
| `struct_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 3.6 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 4.4 |
| `struct_malloc2` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in dereference of `loo | 4.0 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `X` in this scope | 4.0 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1501898) panicked a | 4.8 |
| `struct_multiple` | ❌ FAIL | ❌ error: visibility `pub` is not followed by an item | 3.5 |
| `struct_multiple____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 3.5 |
| `struct_nested` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 4.0 |
| `struct_nested____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 3.5 |
| `struct_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 3.7 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 3.6 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 3.4 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Loom tests passed | 3.6 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0425]: cannot find value `SHARED_STATE` in this sco | 3.6 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 3.6 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 4.7 |
| `unused_func` | ✅ PASS | ✅ Loom tests passed | 3.6 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1508243) panicked a | 4.6 |

## Failures

### array_const____deadlock

```
Message: ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't implement `std::fmt::Display`

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### array_const____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### array_main

```
Message: ❌ error[E0425]: cannot find value `num_mutex` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### array_main____lock_leak

```
Message: ❌ error[E0425]: cannot find value `num_mutex` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
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

### array_main____partial_critical_section

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### array_simple

```
Message: ❌ error[E0428]: the name `f1` is defined multiple times

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
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

### array_simple____partial_critical_section

```
Message: ❌ error[E0425]: cannot find value `n1` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_assume2

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_assume2____self_lock

```
Message: ❌ error: expected one of `:`, `@`, or `|`, found `)`

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_check

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_check____lock_leak

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_check____lock_mismatch

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
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

### global_condvar

```
Message: ❌ thread 'main::test_concurrent_access' (1481731) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_condvar____lost_wakeup

```
Message: ❌ error[E0425]: cannot find value `N1` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_condvar____partial_critical_section

```
Message: ❌ thread 'main::test_concurrent_access' (1482758) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_custom

```
Message: ❌ error[E0428]: the name `lock` is defined multiple times

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_custom____self_lock

```
Message: ❌ error[E0277]: `loom::sync::MutexGuard<'_, i32>` doesn't implement `std::fmt::Display`

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
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

### global_main

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_main____self_lock

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_nested____deadlock

```
Message: ❌ error[E0425]: cannot find value `handle2` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_read____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_rwlock

```
Message: ❌ error[E0425]: cannot find value `LOCK` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_rwlock____lock_leak

```
Message: ❌ error[E0428]: the name `f1` is defined multiple times

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_simple____partial_critical_section

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### global_while

```
Message: ⏱️  Timeout (>600s)

Details:
Loom state space exploration exceeded timeout
```

### global_while____lock_leak

```
Message: ❌ thread 'main::test_concurrent_access' (1493971) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
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
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_alias____self_lock

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_assume

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_assume____deadlock

```
Message: ❌ thread 'main::test_concurrent_access' (1496199) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_condvar

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_condvar____lost_wakeup

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_dup

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_dup____deadlock

```
Message: ❌ error: prefix `pub` is unknown

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_empty

```
Message: ❌ error[E0596]: cannot borrow `s1` as mutable, as it is not declared as mutable

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_init

```
Message: ❌ error[E0614]: type `Ss` cannot be dereferenced

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_init____partial_critical_section

```
Message: ❌ error: cast cannot be followed by a method call

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_main

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_malloc

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_malloc2

```
Message: ❌ error[E0594]: cannot assign to data in dereference of `loom::sync::Arc<Ss>`

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_malloc2____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `X` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_malloc____lost_wakeup

```
Message: ❌ thread 'main::test_concurrent_access' (1501898) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_multiple

```
Message: ❌ error: visibility `pub` is not followed by an item

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_multiple____deadlock

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_nested

```
Message: ❌ error[E0425]: cannot find value `S` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_nested____self_lock

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_simple____partial_critical_section

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_spin

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_timedwait

```
Message: ❌ error[E0425]: cannot find value `SHARED_STATE` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_timedwait____deadlock

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
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

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0425]: cannot find value `S` in this scope

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

### unused_func____lock_mismatch

```
Message: ❌ thread 'main::test_concurrent_access' (1508243) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling pin-project-lite v0.2.17
   Compiling lazy_static v1.5.0
   Compiling nu-ansi-term v0.50.3
   Compiling smallvec v1.15.1
   Compiling scoped-tls v1.0.1
   Compiling
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 3.8 |
| `global_assume` | 3.5 |
| `global_assume____lock_leak` | 3.5 |
| `global_nested` | 3.4 |
| `global_read` | 3.6 |
| `global_simple` | 3.3 |
| `global_trylock` | 3.5 |
| `struct_spin____lock_leak` | 3.6 |
| `unused_func` | 3.6 |
