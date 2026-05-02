# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T12:58:14.308852
**Total Examples:** 62
**Passed (No UB):** 62
**Failed (UB Detected):** 0

## Summary

- **Clean Code Rate:** 62/62 (100.0%)
- **Total Time:** 631.0s
- **Average Time:** 10.2s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 14.4 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.9 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `array_main____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.9 |
| `array_main____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.0 |
| `array_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.9 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `global_assume2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.9 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.0 |
| `global_check` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `global_check____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 20.5 |
| `global_check____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.5 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.6 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.9 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.1 |
| `global_custom____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.5 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.5 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.2 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.8 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.1 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `global_read____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `global_rwlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.2 |
| `global_rwlock____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 28.7 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 19.5 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 17.0 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.7 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.5 |
| `global_while____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.3 |
| `struct_alias` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 35.6 |
| `struct_alias____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.4 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.8 |
| `struct_assume____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.9 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.5 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.5 |
| `struct_dup____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.8 |
| `struct_empty` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.1 |
| `struct_init` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.8 |
| `struct_init____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.6 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 18.5 |
| `struct_malloc` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.3 |
| `struct_malloc2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.4 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.4 |
| `struct_malloc____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.8 |
| `struct_multiple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.6 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 30.8 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.1 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 28.9 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.0 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.6 |
| `struct_spin` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.8 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.4 |
| `struct_timedwait` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.2 |
| `struct_timedwait____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 35.0 |
| `struct_timedwait____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.3 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.2 |
| `unused_func____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const` | 14.4 |
| `array_const____deadlock` | 4.4 |
| `array_const____lock_mismatch` | 8.9 |
| `array_main` | 3.6 |
| `array_main____lock_leak` | 7.9 |
| `array_main____partial_critical_section` | 3.0 |
| `array_simple` | 2.1 |
| `array_simple____partial_critical_section` | 6.9 |
| `global_assume` | 3.5 |
| `global_assume2` | 8.9 |
| `global_assume2____self_lock` | 2.7 |
| `global_assume____lock_leak` | 12.0 |
| `global_check` | 3.5 |
| `global_check____lock_leak` | 20.5 |
| `global_check____lock_mismatch` | 3.8 |
| `global_condvar` | 9.5 |
| `global_condvar____lost_wakeup` | 2.6 |
| `global_condvar____partial_critical_section` | 1.9 |
| `global_custom` | 9.1 |
| `global_custom____self_lock` | 4.5 |
| `global_main` | 5.5 |
| `global_main____self_lock` | 9.2 |
| `global_nested` | 7.8 |
| `global_nested____deadlock` | 3.1 |
| `global_read` | 5.0 |
| `global_read____lock_mismatch` | 3.7 |
| `global_rwlock` | 7.2 |
| `global_rwlock____lock_leak` | 28.7 |
| `global_simple` | 19.5 |
| `global_simple____partial_critical_section` | 17.0 |
| `global_trylock` | 7.7 |
| `global_while` | 6.5 |
| `global_while____lock_leak` | 6.3 |
| `struct_alias` | 35.6 |
| `struct_alias____self_lock` | 8.4 |
| `struct_assume` | 10.8 |
| `struct_assume____deadlock` | 8.9 |
| `struct_condvar` | 3.7 |
| `struct_condvar____lost_wakeup` | 11.5 |
| `struct_dup` | 11.5 |
| `struct_dup____deadlock` | 10.8 |
| `struct_empty` | 3.1 |
| `struct_init` | 11.8 |
| `struct_init____partial_critical_section` | 13.6 |
| `struct_main` | 18.5 |
| `struct_malloc` | 9.3 |
| `struct_malloc2` | 8.4 |
| `struct_malloc2____lock_mismatch` | 8.4 |
| `struct_malloc____lost_wakeup` | 6.8 |
| `struct_multiple` | 7.6 |
| `struct_multiple____deadlock` | 30.8 |
| `struct_nested` | 11.1 |
| `struct_nested____self_lock` | 28.9 |
| `struct_simple` | 13.0 |
| `struct_simple____partial_critical_section` | 8.6 |
| `struct_spin` | 9.8 |
| `struct_spin____lock_leak` | 11.4 |
| `struct_timedwait` | 12.2 |
| `struct_timedwait____deadlock` | 35.0 |
| `struct_timedwait____lost_wakeup` | 8.3 |
| `unused_func` | 8.2 |
| `unused_func____lock_mismatch` | 3.7 |
