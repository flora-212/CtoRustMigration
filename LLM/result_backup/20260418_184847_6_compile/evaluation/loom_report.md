# Loom Concurrency Test Report

**Generated:** 2026-04-21T12:47:43.695922
**Total Examples:** 62
**Passed:** 9
**Failed:** 53

## Summary

- **Pass Rate:** 9/62 (14.5%)
- **Total Time:** 924.9s
- **Average Time:** 14.9s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `loo | 11.1 |
| `array_const____deadlock` | ✅ PASS | ✅ Loom tests passed | 11.9 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Loom tests passed | 11.1 |
| `array_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 17.3 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 11.8 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `num_mutex` in this scope | 14.1 |
| `array_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 11.8 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 16.9 |
| `global_assume` | ✅ PASS | ✅ Loom tests passed | 15.5 |
| `global_assume2` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 17.2 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error: expected one of `:`, `@`, or `\|`, found `)` | 17.3 |
| `global_assume____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `N1` in this scope | 17.2 |
| `global_check` | ❌ FAIL | ❌ error[E0425]: cannot find value `N` in this scope | 18.0 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error[E0382]: use of moved value: `m` | 18.0 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 16.5 |
| `global_condvar` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1698405) panicked a | 18.2 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `COND` in this scope | 16.4 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Loom tests passed | 18.3 |
| `global_custom` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 16.4 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0599]: no method named `unwrap` found for unit type | 15.8 |
| `global_main` | ❌ FAIL | ❌ error: expected one of `!` or `::`, found `init` | 23.3 |
| `global_main____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope | 14.5 |
| `global_nested` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 35.3 |
| `global_nested____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 12.3 |
| `global_read` | ✅ PASS | ✅ Loom tests passed | 14.6 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `n1` in this scope | 11.0 |
| `global_rwlock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 11.0 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find value `LOCK` in this scope | 12.5 |
| `global_simple` | ✅ PASS | ✅ Loom tests passed | 14.3 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Loom tests passed | 11.5 |
| `global_trylock` | ✅ PASS | ✅ Loom tests passed | 11.4 |
| `global_while` | ❌ FAIL | ❌ error: prefix `pub` is unknown | 11.2 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error[E0382]: use of moved value: `shared_data` | 16.2 |
| `struct_alias` | ❌ FAIL | ❌ error[E0502]: cannot borrow `*s` as mutable because it is  | 13.4 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 12.3 |
| `struct_assume` | ❌ FAIL | ❌ error[E0502]: cannot borrow `*s` as mutable because it is  | 14.7 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in dereference of `loo | 20.3 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 17.0 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 16.0 |
| `struct_dup` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1779579) panicked a | 18.8 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 12.1 |
| `struct_empty` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 12.0 |
| `struct_init` | ❌ FAIL | ❌ error[E0425]: cannot find type `Once` in this scope | 18.0 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error[E0428]: the name `t_fun` is defined multiple times | 10.8 |
| `struct_main` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 14.3 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 15.3 |
| `struct_malloc2` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in dereference of `loo | 10.0 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `X` in this scope | 13.0 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in dereference of `loo | 12.2 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0425]: cannot find value `S1` in this scope | 13.2 |
| `struct_multiple____deadlock` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 11.0 |
| `struct_nested` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 12.8 |
| `struct_nested____self_lock` | ❌ FAIL | ❌ thread 'main::test_concurrent_access' (1813350) panicked a | 17.0 |
| `struct_simple` | ❌ FAIL | ❌ error[E0425]: cannot find value `INIT` in this scope | 14.8 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0425]: cannot find value `S` in this scope | 16.5 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 12.4 |
| `struct_spin____lock_leak` | ❌ FAIL | ❌ error: test failed, to rerun pass `--lib` | 18.5 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0425]: cannot find value `SHARED_STATE` in this sco | 17.5 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 11.9 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 11.4 |
| `unused_func` | ✅ PASS | ✅ Loom tests passed | 11.2 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope | 16.9 |

## Failures

### array_const

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
   Compiling cfg-if v1.0.4
   
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
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling libc v0.
```

### array_main____lock_leak

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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   
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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   
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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   
```

### global_assume2

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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Co
```

### global_assume____lock_leak

```
Message: ❌ error[E0425]: cannot find value `N1` in this scope

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

### global_check

```
Message: ❌ error[E0425]: cannot find value `N` in this scope

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
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling pin-proj
```

### global_check____lock_leak

```
Message: ❌ error[E0382]: use of moved value: `m`

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

### global_check____lock_mismatch

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
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling pin-proj
```

### global_condvar

```
Message: ❌ thread 'main::test_concurrent_access' (1698405) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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

### global_condvar____lost_wakeup

```
Message: ❌ error[E0425]: cannot find value `COND` in this scope

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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8
```

### global_custom____self_lock

```
Message: ❌ error[E0599]: no method named `unwrap` found for unit type `()` in the current scope

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
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.4
   
```

### global_main____self_lock

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX` in this scope

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
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling cfg-if v1.
```

### global_nested

```
Message: ❌ error[E0425]: cannot find value `INIT` in this scope

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
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4.29
   Compiling pin-proj
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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   
```

### global_read____lock_mismatch

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

### global_rwlock

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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Co
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
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling log v0.4
```

### global_while____lock_leak

```
Message: ❌ error[E0382]: use of moved value: `shared_data`

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
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling lazy_sta
```

### struct_alias

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
```

### struct_alias____self_lock

```
Message: ❌ error[E0425]: cannot find value `S1` in this scope

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
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling cfg-if v1.0.4
   
```

### struct_assume____deadlock

```
Message: ❌ error[E0594]: cannot assign to data in dereference of `loom::sync::Arc<Ss>`

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
   Compiling cfg-if v1.0.4
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Co
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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.
```

### struct_dup

```
Message: ❌ thread 'main::test_concurrent_access' (1779579) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling libc v0.2.185
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling log v0.4.29
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8
```

### struct_init

```
Message: ❌ error[E0425]: cannot find type `Once` in this scope

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

### struct_init____partial_critical_section

```
Message: ❌ error[E0428]: the name `t_fun` is defined multiple times

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
   Compiling libc v0.2.185
   Compiling log v0.4.29
   
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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
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
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.
```

### struct_malloc2

```
Message: ❌ error[E0594]: cannot assign to data in dereference of `loom::sync::Arc<Ss>`

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
   Compiling log v0.4.29
   Compiling regex-syntax v0.8
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
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling find-msvc-tools v0.1.9
   Compiling libc v0.2.185
   
```

### struct_multiple

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
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
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
   Compiling rustversion v1.0.22
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling regex-syntax v0.8.10
   Compiling libc v0.2.185
   Compiling log v0.4
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling shlex v1.3.0
   Compiling regex-syntax v0.8
```

### struct_nested____self_lock

```
Message: ❌ thread 'main::test_concurrent_access' (1813350) panicked at /rustc/c756124775121dea0e640652c5ee3c89e

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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Co
```

### struct_simple____partial_critical_section

```
Message: ❌ error[E0425]: cannot find value `S` in this scope

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
   Compiling log v0.4.29
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling lazy_sta
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
   Compiling rustversion v1.0.22
   Compiling find-msvc-tools v0.1.9
   Compiling shlex v1.3.0
   Compiling once_cell v1.21.4
   Compiling libc v0.2.185
   Compiling regex-syntax v0.8.10
   Compiling log v0.4
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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling regex-syntax v0.8.10
   Compiling cfg-if v1.0.4
   Compiling libc v0.
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
    Blocking waiting for file lock on package cache
   Compiling rustversion v1.0.22
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Co
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
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.9
   Compiling once_cell v1.21.4
   Compiling log v0.4.29
   Co
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
   Compiling log v0.4.29
   Co
```

### unused_func____lock_mismatch

```
Message: ❌ error[E0425]: cannot find value `NUM_MUTEX1` in this scope

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
   Compiling libc v0.2.185
   Compiling cfg-if v1.0.4
   Compiling regex-syntax v0
```

## Passed Examples

| Example | Time (s) |
|---------|----------|
| `array_const____deadlock` | 11.9 |
| `array_const____lock_mismatch` | 11.1 |
| `global_assume` | 15.5 |
| `global_condvar____partial_critical_section` | 18.3 |
| `global_read` | 14.6 |
| `global_simple` | 14.3 |
| `global_simple____partial_critical_section` | 11.5 |
| `global_trylock` | 11.4 |
| `unused_func` | 11.2 |
