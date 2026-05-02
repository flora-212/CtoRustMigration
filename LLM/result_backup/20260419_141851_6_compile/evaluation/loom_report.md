# Loom Concurrency Test Report

**Generated:** 2026-04-21T12:49:42.670175
**Total Examples:** 62
**Passed:** 9
**Failed:** 53

## Summary

- **Pass Rate:** 9/62 (14.5%)
- **Total Time:** 922.4s
- **Average Time:** 14.9s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Loom tests passed | 18.3 |
| `array_const____deadlock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 17.1 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error[E0594]: cannot assign to `self.values[_]`, which is  | 17.1 |
| `array_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 17.4 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 17.6 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 16.7 |
| `array_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 17.3 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error: mismatched closing delimiter: `)` | 17.2 |
| `global_assume` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 18.3 |
| `global_assume2` | ✅ PASS | ✅ Loom tests passed | 17.4 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `N1` in this scope | 15.5 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Loom tests passed | 23.2 |
| `global_check` | ❌ FAIL | ❌ error[E0425]: cannot find value `GLOBAL_MUTEX` in this sco | 13.9 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `M` in this scope | 35.0 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `N` in this scope | 13.8 |
| `global_condvar` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1729120) panicked a | 15.2 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1732989) panicked a | 13.5 |
| `global_condvar____partial_critical_section` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1736489) panicked a | 10.9 |
| `global_custom` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 8.8 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0596]: cannot borrow `guard` as mutable, as it is n | 13.5 |
| `global_main` | ✅ PASS | ✅ Loom tests passed | 12.1 |
| `global_main____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 10.6 |
| `global_nested` | ❌ FAIL | ❌ error[E0425]: cannot find value `SHARED_DATA` in this scop | 12.3 |
| `global_nested____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `handle2` in this scope | 15.5 |
| `global_read` | ✅ PASS | ✅ Loom tests passed | 14.6 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 10.8 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 15.3 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 20.3 |
| `global_simple` | ✅ PASS | ✅ Loom tests passed | 16.8 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Loom tests passed | 16.2 |
| `global_trylock` | ✅ PASS | ✅ Loom tests passed | 16.5 |
| `global_while` | ❌ FAIL | ❌ error[E0425]: cannot find value `N1` in this scope | 13.7 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 12.5 |
| `struct_alias` | ❌ FAIL | ❌ error[E0428]: the name `f1_loom_wrapper` is defined multip | 16.9 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 11.8 |
| `struct_assume` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 14.9 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for reference ` | 14.4 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 10.5 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 12.7 |
| `struct_dup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 13.1 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 12.6 |
| `struct_empty` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 11.2 |
| `struct_init` | ❌ FAIL | ❌ error[E0308]: mismatched types | 12.1 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error[E0428]: the name `t_fun` is defined multiple times | 16.3 |
| `struct_main` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1817042) panicked a | 17.8 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 14.5 |
| `struct_malloc2` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 12.9 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `X` in this scope | 18.3 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0277]: `ss` doesn't implement `Debug` | 18.0 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0428]: the name `f1_loom_wrapper` is defined multip | 10.9 |
| `struct_multiple____deadlock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 12.0 |
| `struct_nested` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 10.3 |
| `struct_nested____self_lock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 4.2 |
| `struct_simple` | ❌ FAIL | ❌ error[E0599]: no method named `as_ptr` found for struct `l | 13.2 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find type `Once` in this scope | 17.6 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 14.0 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Loom tests passed | 11.5 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 11.2 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `SHARED_DATA` in this scop | 12.0 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `s` in this scope | 15.4 |
| `unused_func` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 23.8 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope | 13.3 |

## Failures

### array_const____deadlock

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8
```

### array_const____lock_mismatch

```
Message: ❌ error[E0594]: cannot assign to `self.values[_]`, which is behind a `&` reference

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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   
```

### array_main____lock_leak

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   
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
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling pin-proj
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
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### array_simple____partial_critical_section

```
Message: ❌ error: mismatched closing delimiter: `)`

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
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8
```

### global_assume

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
   Compiling log v0.4.29
   Co
```

### global_assume2____self_lock

```
Message: ❌ error[E0425]: cannot find value `N1` in this scope

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
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling lazy_sta
```

### global_check

```
Message: ❌ error[E0425]: cannot find value `GLOBAL_MUTEX` in this scope

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

### global_check____lock_leak

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
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.
```

### global_check____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `N` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
```

### global_condvar

```
Message: ❌ thread 'main::test_concurrent_access' (1729120) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0
```

### global_condvar____lost_wakeup

```
Message: ❌ thread 'main::test_concurrent_access' (1732989) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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

### global_condvar____partial_critical_section

```
Message: ❌ thread 'main::test_concurrent_access' (1736489) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
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

### global_custom____self_lock

```
Message: ❌ error[E0596]: cannot borrow `guard` as mutable, as it is not declared as mutable

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
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.
```

### global_nested

```
Message: ❌ error[E0425]: cannot find value `SHARED_DATA` in this scope

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4
```

### global_read____lock_mismatch

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
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
```

### global_rwlock

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
   Compiling libc v0.2.185
   
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling pin-proj
```

### global_while

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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8
```

### global_while____lock_leak

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8
```

### struct_alias

```
Message: ❌ error[E0428]: the name `f1_loom_wrapper` is defined multiple times

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
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling pin-proj
```

### struct_alias____self_lock

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0
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
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
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
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   
```

### struct_condvar

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling libc v0.2.
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Co
```

### struct_dup

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
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
```

### struct_empty

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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
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
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   
```

### struct_init____partial_critical_section

```
Message: ❌ error[E0428]: the name `t_fun` is defined multiple times

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
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   
```

### struct_main

```
Message: ❌ thread 'main::test_concurrent_access' (1817042) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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

### struct_malloc2

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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling lazy_sta
```

### struct_malloc2____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `X` in this scope

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

### struct_malloc____lost_wakeup

```
Message: ❌ error[E0277]: `ss` doesn't implement `Debug`

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
   Compiling libc v0.2.185
   
```

### struct_multiple

```
Message: ❌ error[E0428]: the name `f1_loom_wrapper` is defined multiple times

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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.
```

### struct_nested

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
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   
```

### struct_nested____self_lock

```
Message: ❌ error: test failed, to rerun pass `--lib`

Details:
    Updating crates.io index
     Locking 30 packages to latest compatible versions
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.17
   Compiling smallvec v1.15.1
   Compiling nu-ansi-term v0.50.3
   Compiling scoped-tls v1.0.1
   Compiling
```

### struct_simple

```
Message: ❌ error[E0599]: no method named `as_ptr` found for struct `loom::sync::Arc<SharedData>` in the current

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

### struct_simple____partial_critical_section

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   
```

### struct_timedwait

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
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling cfg-if v
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0425]: cannot find value `s` in this scope

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

### unused_func

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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling libc v0.2.185
   
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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const` | 18.3 |
| `global_assume2` | 17.4 |
| `global_assume____lock_leak` | 23.2 |
| `global_main` | 12.1 |
| `global_read` | 14.6 |
| `global_simple` | 16.8 |
| `global_simple____partial_critical_section` | 16.2 |
| `global_trylock` | 16.5 |
| `struct_spin____lock_leak` | 11.5 |
