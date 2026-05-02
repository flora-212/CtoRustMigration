# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T13:02:44.018648
**Total Examples:** 62
**Passed (No UB):** 61
**Failed (UB Detected):** 1

## Summary

- **Clean Code Rate:** 61/62 (98.4%)
- **Total Time:** 507.5s
- **Average Time:** 8.2s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 16.2 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.6 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.4 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.1 |
| `array_main____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.8 |
| `array_main____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `array_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.9 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 31.0 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.3 |
| `global_assume2` | ❌ FAIL | ❌ error[E0308]: mismatched types | 7.3 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 29.5 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 14.0 |
| `global_check` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.4 |
| `global_check____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.5 |
| `global_check____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.3 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.6 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 35.1 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.0 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.4 |
| `global_custom____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.9 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.0 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.8 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.1 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.0 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 26.4 |
| `global_read____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.9 |
| `global_rwlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.6 |
| `global_rwlock____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.3 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.1 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.2 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.0 |
| `global_while____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.1 |
| `struct_alias` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `struct_alias____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.5 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.1 |
| `struct_assume____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.7 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.6 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 24.5 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `struct_dup____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.6 |
| `struct_empty` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `struct_init` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `struct_init____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.9 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.5 |
| `struct_malloc` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `struct_malloc2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.0 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.1 |
| `struct_malloc____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `struct_multiple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 50.0 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.2 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.3 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.9 |
| `struct_spin` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.6 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.0 |
| `struct_timedwait` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.8 |
| `struct_timedwait____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 16.1 |
| `struct_timedwait____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.6 |
| `unused_func____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.1 |

## UB Detected (Failures)

### global_assume2

```
Message: ❌ error[E0308]: mismatched types

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume2_d6dvgr
```

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const` | 16.2 |
| `array_const____deadlock` | 9.6 |
| `array_const____lock_mismatch` | 8.4 |
| `array_main` | 8.1 |
| `array_main____lock_leak` | 6.8 |
| `array_main____partial_critical_section` | 3.7 |
| `array_simple` | 3.9 |
| `array_simple____partial_critical_section` | 31.0 |
| `global_assume` | 2.3 |
| `global_assume2____self_lock` | 29.5 |
| `global_assume____lock_leak` | 14.0 |
| `global_check` | 8.4 |
| `global_check____lock_leak` | 9.5 |
| `global_check____lock_mismatch` | 11.3 |
| `global_condvar` | 12.6 |
| `global_condvar____lost_wakeup` | 35.1 |
| `global_condvar____partial_critical_section` | 8.0 |
| `global_custom` | 8.4 |
| `global_custom____self_lock` | 3.9 |
| `global_main` | 6.0 |
| `global_main____self_lock` | 6.8 |
| `global_nested` | 7.1 |
| `global_nested____deadlock` | 6.0 |
| `global_read` | 26.4 |
| `global_read____lock_mismatch` | 5.9 |
| `global_rwlock` | 4.6 |
| `global_rwlock____lock_leak` | 5.3 |
| `global_simple` | 7.1 |
| `global_simple____partial_critical_section` | 3.6 |
| `global_trylock` | 4.2 |
| `global_while` | 6.0 |
| `global_while____lock_leak` | 4.1 |
| `struct_alias` | 4.4 |
| `struct_alias____self_lock` | 4.5 |
| `struct_assume` | 4.1 |
| `struct_assume____deadlock` | 4.7 |
| `struct_condvar` | 5.6 |
| `struct_condvar____lost_wakeup` | 24.5 |
| `struct_dup` | 3.2 |
| `struct_dup____deadlock` | 4.6 |
| `struct_empty` | 3.6 |
| `struct_init` | 2.7 |
| `struct_init____partial_critical_section` | 2.9 |
| `struct_main` | 2.5 |
| `struct_malloc` | 3.2 |
| `struct_malloc2` | 3.0 |
| `struct_malloc2____lock_mismatch` | 3.1 |
| `struct_malloc____lost_wakeup` | 3.2 |
| `struct_multiple` | 50.0 |
| `struct_multiple____deadlock` | 2.2 |
| `struct_nested` | 3.3 |
| `struct_nested____self_lock` | 2.1 |
| `struct_simple` | 2.4 |
| `struct_simple____partial_critical_section` | 2.9 |
| `struct_spin` | 2.6 |
| `struct_spin____lock_leak` | 2.0 |
| `struct_timedwait` | 1.8 |
| `struct_timedwait____deadlock` | 16.1 |
| `struct_timedwait____lost_wakeup` | 2.4 |
| `unused_func` | 4.6 |
| `unused_func____lock_mismatch` | 4.1 |
