# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T13:00:05.866629
**Total Examples:** 62
**Passed (No UB):** 61
**Failed (UB Detected):** 1

## Summary

- **Clean Code Rate:** 61/62 (98.4%)
- **Total Time:** 609.7s
- **Average Time:** 9.8s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.3 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.9 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.5 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `array_main____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.2 |
| `array_main____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `array_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.9 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 29.1 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 18.8 |
| `global_assume2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 17.4 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.6 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.4 |
| `global_check` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.8 |
| `global_check____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.6 |
| `global_check____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.6 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 35.3 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.3 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.4 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.3 |
| `global_custom____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.2 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.6 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.8 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.9 |
| `global_read____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.8 |
| `global_rwlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 18.9 |
| `global_rwlock____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.7 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.4 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.0 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.2 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.0 |
| `global_while____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.9 |
| `struct_alias` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 30.6 |
| `struct_alias____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.6 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 28.6 |
| `struct_assume____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.8 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.7 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.1 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.2 |
| `struct_dup____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.7 |
| `struct_empty` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 34.7 |
| `struct_init` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.8 |
| `struct_init____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.3 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.4 |
| `struct_malloc` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.1 |
| `struct_malloc2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.0 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.7 |
| `struct_malloc____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.4 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0277]: `St` doesn't implement `std::fmt::Debug` | 26.0 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.4 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.8 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.2 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.7 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.9 |
| `struct_spin` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.5 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.3 |
| `struct_timedwait` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.5 |
| `struct_timedwait____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.3 |
| `struct_timedwait____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.5 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `unused_func____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |

## UB Detected (Failures)

### struct_multiple

```
Message: ❌ error[E0277]: `St` doesn't implement `std::fmt::Debug`

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_multiple__q_a1c8w)
    Finished `test` profile [unoptimized + debu
```

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const` | 6.3 |
| `array_const____deadlock` | 8.9 |
| `array_const____lock_mismatch` | 8.5 |
| `array_main` | 2.7 |
| `array_main____lock_leak` | 5.2 |
| `array_main____partial_critical_section` | 3.6 |
| `array_simple` | 6.9 |
| `array_simple____partial_critical_section` | 29.1 |
| `global_assume` | 18.8 |
| `global_assume2` | 17.4 |
| `global_assume2____self_lock` | 4.6 |
| `global_assume____lock_leak` | 3.4 |
| `global_check` | 1.8 |
| `global_check____lock_leak` | 4.6 |
| `global_check____lock_mismatch` | 6.6 |
| `global_condvar` | 35.3 |
| `global_condvar____lost_wakeup` | 8.3 |
| `global_condvar____partial_critical_section` | 10.4 |
| `global_custom` | 8.3 |
| `global_custom____self_lock` | 5.0 |
| `global_main` | 11.2 |
| `global_main____self_lock` | 11.6 |
| `global_nested` | 10.8 |
| `global_nested____deadlock` | 3.5 |
| `global_read` | 11.9 |
| `global_read____lock_mismatch` | 12.8 |
| `global_rwlock` | 18.9 |
| `global_rwlock____lock_leak` | 9.7 |
| `global_simple` | 8.4 |
| `global_simple____partial_critical_section` | 8.0 |
| `global_trylock` | 4.2 |
| `global_while` | 3.0 |
| `global_while____lock_leak` | 7.9 |
| `struct_alias` | 30.6 |
| `struct_alias____self_lock` | 10.6 |
| `struct_assume` | 28.6 |
| `struct_assume____deadlock` | 13.8 |
| `struct_condvar` | 8.7 |
| `struct_condvar____lost_wakeup` | 9.1 |
| `struct_dup` | 12.2 |
| `struct_dup____deadlock` | 11.7 |
| `struct_empty` | 34.7 |
| `struct_init` | 8.8 |
| `struct_init____partial_critical_section` | 8.3 |
| `struct_main` | 3.4 |
| `struct_malloc` | 6.1 |
| `struct_malloc2` | 7.0 |
| `struct_malloc2____lock_mismatch` | 7.7 |
| `struct_malloc____lost_wakeup` | 5.4 |
| `struct_multiple____deadlock` | 6.4 |
| `struct_nested` | 4.8 |
| `struct_nested____self_lock` | 5.2 |
| `struct_simple` | 6.7 |
| `struct_simple____partial_critical_section` | 3.9 |
| `struct_spin` | 4.5 |
| `struct_spin____lock_leak` | 5.3 |
| `struct_timedwait` | 4.5 |
| `struct_timedwait____deadlock` | 4.3 |
| `struct_timedwait____lost_wakeup` | 4.5 |
| `unused_func` | 4.4 |
| `unused_func____lock_mismatch` | 4.9 |
