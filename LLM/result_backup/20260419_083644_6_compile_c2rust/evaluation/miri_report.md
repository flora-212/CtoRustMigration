# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T13:04:59.112679
**Total Examples:** 62
**Passed (No UB):** 62
**Failed (UB Detected):** 0

## Summary

- **Clean Code Rate:** 62/62 (100.0%)
- **Total Time:** 375.9s
- **Average Time:** 6.1s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 23.9 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.6 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |
| `array_main____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.4 |
| `array_main____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.9 |
| `array_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.9 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |
| `global_assume2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.0 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.5 |
| `global_check` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.7 |
| `global_check____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.3 |
| `global_check____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 24.2 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.4 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.5 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `global_custom____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.0 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.1 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `global_read____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 50.2 |
| `global_rwlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.6 |
| `global_rwlock____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.8 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.9 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.9 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.6 |
| `global_while____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `struct_alias` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `struct_alias____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 15.9 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.5 |
| `struct_assume____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.9 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 18.7 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `struct_dup____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `struct_empty` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.5 |
| `struct_init` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 20.8 |
| `struct_init____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 26.1 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.3 |
| `struct_malloc` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `struct_malloc2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.6 |
| `struct_malloc____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `struct_multiple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.6 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.4 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.1 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.8 |
| `struct_spin` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `struct_timedwait` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.3 |
| `struct_timedwait____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.6 |
| `struct_timedwait____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.6 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `unused_func____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const` | 23.9 |
| `array_const____deadlock` | 5.6 |
| `array_const____lock_mismatch` | 5.0 |
| `array_main` | 4.9 |
| `array_main____lock_leak` | 7.4 |
| `array_main____partial_critical_section` | 3.9 |
| `array_simple` | 3.7 |
| `array_simple____partial_critical_section` | 5.9 |
| `global_assume` | 4.9 |
| `global_assume2` | 4.0 |
| `global_assume2____self_lock` | 4.4 |
| `global_assume____lock_leak` | 4.5 |
| `global_check` | 4.7 |
| `global_check____lock_leak` | 5.3 |
| `global_check____lock_mismatch` | 24.2 |
| `global_condvar` | 3.4 |
| `global_condvar____lost_wakeup` | 4.5 |
| `global_condvar____partial_critical_section` | 3.5 |
| `global_custom` | 2.7 |
| `global_custom____self_lock` | 3.0 |
| `global_main` | 2.4 |
| `global_main____self_lock` | 3.5 |
| `global_nested` | 2.7 |
| `global_nested____deadlock` | 3.1 |
| `global_read` | 3.2 |
| `global_read____lock_mismatch` | 50.2 |
| `global_rwlock` | 1.6 |
| `global_rwlock____lock_leak` | 4.4 |
| `global_simple` | 1.8 |
| `global_simple____partial_critical_section` | 1.9 |
| `global_trylock` | 2.9 |
| `global_while` | 2.6 |
| `global_while____lock_leak` | 2.4 |
| `struct_alias` | 1.5 |
| `struct_alias____self_lock` | 15.9 |
| `struct_assume` | 2.5 |
| `struct_assume____deadlock` | 4.9 |
| `struct_condvar` | 3.9 |
| `struct_condvar____lost_wakeup` | 18.7 |
| `struct_dup` | 3.6 |
| `struct_dup____deadlock` | 3.8 |
| `struct_empty` | 6.5 |
| `struct_init` | 20.8 |
| `struct_init____partial_critical_section` | 26.1 |
| `struct_main` | 5.3 |
| `struct_malloc` | 3.8 |
| `struct_malloc2` | 2.7 |
| `struct_malloc2____lock_mismatch` | 2.6 |
| `struct_malloc____lost_wakeup` | 3.2 |
| `struct_multiple` | 2.6 |
| `struct_multiple____deadlock` | 2.7 |
| `struct_nested` | 3.4 |
| `struct_nested____self_lock` | 2.4 |
| `struct_simple` | 3.1 |
| `struct_simple____partial_critical_section` | 6.8 |
| `struct_spin` | 2.7 |
| `struct_spin____lock_leak` | 5.0 |
| `struct_timedwait` | 2.3 |
| `struct_timedwait____deadlock` | 1.6 |
| `struct_timedwait____lost_wakeup` | 1.6 |
| `unused_func` | 2.4 |
| `unused_func____lock_mismatch` | 1.4 |
