# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T12:59:22.575630
**Total Examples:** 62
**Passed (No UB):** 62
**Failed (UB Detected):** 0

## Summary

- **Clean Code Rate:** 62/62 (100.0%)
- **Total Time:** 654.5s
- **Average Time:** 10.6s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.2 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.6 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.4 |
| `array_main____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.8 |
| `array_main____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `array_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 20.3 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.8 |
| `global_assume2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.4 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.0 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `global_check` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.2 |
| `global_check____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.0 |
| `global_check____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.0 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.3 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.1 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.1 |
| `global_custom____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 28.7 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 18.9 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 17.7 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.6 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.6 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.2 |
| `global_read____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 35.9 |
| `global_rwlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.7 |
| `global_rwlock____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.7 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.0 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.3 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.2 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.7 |
| `global_while____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.4 |
| `struct_alias` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.9 |
| `struct_alias____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.2 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.8 |
| `struct_assume____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 18.8 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.2 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.8 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.8 |
| `struct_dup____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.0 |
| `struct_empty` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.1 |
| `struct_init` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 30.6 |
| `struct_init____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.7 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 29.0 |
| `struct_malloc` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.8 |
| `struct_malloc2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.3 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.0 |
| `struct_malloc____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.8 |
| `struct_multiple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.4 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 34.7 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.5 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.3 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.9 |
| `struct_spin` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.0 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.9 |
| `struct_timedwait` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.5 |
| `struct_timedwait____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 26.1 |
| `struct_timedwait____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.0 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |
| `unused_func____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const` | 6.2 |
| `array_const____deadlock` | 3.7 |
| `array_const____lock_mismatch` | 8.6 |
| `array_main` | 3.4 |
| `array_main____lock_leak` | 11.8 |
| `array_main____partial_critical_section` | 3.7 |
| `array_simple` | 20.3 |
| `array_simple____partial_critical_section` | 3.8 |
| `global_assume` | 9.8 |
| `global_assume2` | 3.4 |
| `global_assume2____self_lock` | 10.0 |
| `global_assume____lock_leak` | 4.4 |
| `global_check` | 5.2 |
| `global_check____lock_leak` | 9.0 |
| `global_check____lock_mismatch` | 9.0 |
| `global_condvar` | 2.3 |
| `global_condvar____lost_wakeup` | 5.1 |
| `global_condvar____partial_critical_section` | 3.8 |
| `global_custom` | 7.1 |
| `global_custom____self_lock` | 28.7 |
| `global_main` | 18.9 |
| `global_main____self_lock` | 17.7 |
| `global_nested` | 7.6 |
| `global_nested____deadlock` | 6.6 |
| `global_read` | 6.2 |
| `global_read____lock_mismatch` | 35.9 |
| `global_rwlock` | 8.7 |
| `global_rwlock____lock_leak` | 9.7 |
| `global_simple` | 8.0 |
| `global_simple____partial_critical_section` | 5.3 |
| `global_trylock` | 11.2 |
| `global_while` | 11.7 |
| `global_while____lock_leak` | 10.4 |
| `struct_alias` | 2.9 |
| `struct_alias____self_lock` | 12.2 |
| `struct_assume` | 13.8 |
| `struct_assume____deadlock` | 18.8 |
| `struct_condvar` | 9.2 |
| `struct_condvar____lost_wakeup` | 8.8 |
| `struct_dup` | 7.8 |
| `struct_dup____deadlock` | 7.0 |
| `struct_empty` | 8.1 |
| `struct_init` | 30.6 |
| `struct_init____partial_critical_section` | 10.7 |
| `struct_main` | 29.0 |
| `struct_malloc` | 13.8 |
| `struct_malloc2` | 8.3 |
| `struct_malloc2____lock_mismatch` | 9.0 |
| `struct_malloc____lost_wakeup` | 11.8 |
| `struct_multiple` | 12.4 |
| `struct_multiple____deadlock` | 34.7 |
| `struct_nested` | 8.5 |
| `struct_nested____self_lock` | 8.3 |
| `struct_simple` | 3.8 |
| `struct_simple____partial_critical_section` | 5.9 |
| `struct_spin` | 7.0 |
| `struct_spin____lock_leak` | 6.9 |
| `struct_timedwait` | 6.5 |
| `struct_timedwait____deadlock` | 26.1 |
| `struct_timedwait____lost_wakeup` | 6.0 |
| `unused_func` | 4.9 |
| `unused_func____lock_mismatch` | 4.9 |
