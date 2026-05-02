# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T12:59:42.685785
**Total Examples:** 62
**Passed (No UB):** 31
**Failed (UB Detected):** 31

## Summary

- **Clean Code Rate:** 31/62 (50.0%)
- **Total Time:** 599.8s
- **Average Time:** 9.7s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.8 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `std | 4.3 |
| `array_main` | ❌ FAIL | ❌ error[E0530]: let bindings cannot shadow statics | 1.9 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 7.2 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 1.3 |
| `array_simple` | ❌ FAIL | ❌ Compiling miri_test v0.1.0 (/tmp/miri_test_array_simple_8h | 7.0 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `global_assume2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.4 |
| `global_check` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `P | 28.3 |
| `global_check____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 19.8 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 16.9 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.0 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.3 |
| `global_custom` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 5.4 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0596]: cannot borrow `guard` as mutable, as it is n | 1.3 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 35.1 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.3 |
| `global_nested` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 9.5 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.5 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.7 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 10.1 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0133]: use of mutable static is unsafe and requires | 11.8 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_rwlock_rdlock` | 10.2 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.1 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.1 |
| `global_while` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `P | 17.9 |
| `global_while____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.4 |
| `struct_alias` | ❌ FAIL | ❌ error[E0308]: mismatched types | 7.5 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 4.8 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.3 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for reference ` | 6.0 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.5 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 30.9 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 10.6 |
| `struct_dup____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 28.4 |
| `struct_empty` | ❌ FAIL | ❌ error[E0308]: mismatched types | 12.9 |
| `struct_init` | ❌ FAIL | ❌ error[E0308]: mismatched types | 8.5 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error[E0596]: cannot borrow `s1` as mutable, as it is not  | 9.7 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.0 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 11.7 |
| `struct_malloc2` | ❌ FAIL | ❌ error[E0308]: mismatched types | 34.6 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0505]: cannot move out of `s` because it is borrowe | 8.3 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0277]: `ss` doesn't implement `std::fmt::Debug` | 8.9 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0277]: `St` doesn't implement `std::fmt::Debug` | 3.7 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.6 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.0 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.0 |
| `struct_simple` | ❌ FAIL | ❌ error[E0599]: no method named `as_ptr` found for struct `s | 5.5 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 27.2 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 5.2 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.3 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0382]: the type `Arc` does not implement `Copy` | 4.3 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `wait_until` found for struc | 7.4 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0599]: no method named `wait_until` found for struc | 3.8 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.4 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `M | 5.2 |

## UB Detected (Failures)

### array_const____lock_mismatch

```
Message: ❌ error[E0599]: no method named `lock` found for struct `std::sync::Arc<MutexArray>` in the current sc

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const____lock_m
```

### array_main

```
Message: ❌ error[E0530]: let bindings cannot shadow statics

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main_lhuz48ue)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.77s
     Running unittests src/main
```

### array_main____lock_leak

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main____lock_le
```

### array_main____partial_critical_section

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main____partial_critical_section_xtlfu61c)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.08s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated 
```

### array_simple

```
Message: ❌ Compiling miri_test v0.1.0 (/tmp/miri_test_array_simple_8hstubfr)

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_simple_8hstubfr)
    Finished `test` profile [unoptimized + debugin
```

### global_check

```
Message: ❌ error[E0015]: cannot call non-const associated function `PthreadMutex::new` in statics

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check_tmlt7gkj)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 28.12s
     Running unittests src/m
```

### global_check____lock_mismatch

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_
```

### global_custom

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_custom_ioeo8av
```

### global_custom____self_lock

```
Message: ❌ error[E0596]: cannot borrow `guard` as mutable, as it is not declared as mutable

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_custom____self
```

### global_nested

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<SharedData>::new` in stati

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_nested_5ckqyah
```

### global_read____lock_mismatch

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_read____lock_mismatch_hluhxo5r)
    Finished `test` profile [unopt
```

### global_rwlock

```
Message: ❌ error[E0133]: use of mutable static is unsafe and requires unsafe function or block

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock_s5wifhyv)
    Finished `test` profile [unoptimized + debugi
```

### global_rwlock____lock_leak

```
Message: ❌ error[E0425]: cannot find function `pthread_rwlock_rdlock` in this scope

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock____lock_leak_a3dd0mm2)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.73s
     Running un
```

### global_while

```
Message: ❌ error[E0015]: cannot call non-const associated function `PthreadMutex::new` in statics

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_while_gzq0m5iy
```

### struct_alias

```
Message: ❌ error[E0308]: mismatched types

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias_d1238yof)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.23s
     Running unittests src/ma
```

### struct_alias____self_lock

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<St>::new` in statics

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias____self_lock_ybi0ofm7)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.60s
     Running uni
```

### struct_assume____deadlock

```
Message: ❌ error[E0599]: no method named `lock` found for reference `&std::sync::Arc<Ss>` in the current scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_assume____dead
```

### struct_empty

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
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_empty_04dzkgpv
```

### struct_init

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init__ttyqv6t)
    Finished `test` profile [unoptimized + debuginf
```

### struct_init____partial_critical_section

```
Message: ❌ error[E0596]: cannot borrow `s1` as mutable, as it is not declared as mutable

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init____partial_critical_section_k81638zm)
    Finished `test` pro
```

### struct_malloc

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc_3sonvfg_)
    Finished `test` profile [unoptimized + debugi
```

### struct_malloc2

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc2_mhws7388)
    Finished `test` profile [unoptimized + debug
```

### struct_malloc2____lock_mismatch

```
Message: ❌ error[E0505]: cannot move out of `s` because it is borrowed

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc2____lock_mismatch_mng1tpik)
    Finished `test` profile [un
```

### struct_malloc____lost_wakeup

```
Message: ❌ error[E0277]: `ss` doesn't implement `std::fmt::Debug`

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc____lost_wakeup_zpzg3t1j)
    Finished `test` profile [unopt
```

### struct_multiple

```
Message: ❌ error[E0277]: `St` doesn't implement `std::fmt::Debug`

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_multiple_hy00s3c6)
    Finished `test` profile [unoptimized + debu
```

### struct_simple

```
Message: ❌ error[E0599]: no method named `as_ptr` found for struct `std::sync::Arc<SharedData>` in the current 

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_simple_o0qzlsm
```

### struct_spin

```
Message: ❌ error[E0308]: mismatched types

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_spin_fpvonkpk)
    Finished `test` profile [unoptimized + debuginf
```

### struct_timedwait

```
Message: ❌ error[E0382]: the type `Arc` does not implement `Copy`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait_gjtk
```

### struct_timedwait____deadlock

```
Message: ❌ error[E0599]: no method named `wait_until` found for struct `std::sync::Condvar` in the current scop

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____deadlock_nt1pol6b)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.21s
     Running 
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0599]: no method named `wait_until` found for struct `std::sync::Condvar` in the current scop

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____lost_wakeup_91_rx6do)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.60s
     Runni
```

### unused_func____lock_mismatch

```
Message: ❌ error[E0015]: cannot call non-const associated function `MutexWrapper::new` in statics

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_unused_func____lock_m
```

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const` | 9.8 |
| `array_const____deadlock` | 4.4 |
| `array_simple____partial_critical_section` | 2.7 |
| `global_assume` | 2.1 |
| `global_assume2` | 4.4 |
| `global_assume2____self_lock` | 3.8 |
| `global_assume____lock_leak` | 6.4 |
| `global_check____lock_leak` | 19.8 |
| `global_condvar` | 5.0 |
| `global_condvar____lost_wakeup` | 4.0 |
| `global_condvar____partial_critical_section` | 6.3 |
| `global_main` | 35.1 |
| `global_main____self_lock` | 8.3 |
| `global_nested____deadlock` | 8.5 |
| `global_read` | 5.7 |
| `global_simple` | 3.7 |
| `global_simple____partial_critical_section` | 13.1 |
| `global_trylock` | 13.1 |
| `global_while____lock_leak` | 10.4 |
| `struct_assume` | 4.3 |
| `struct_condvar` | 8.5 |
| `struct_condvar____lost_wakeup` | 30.9 |
| `struct_dup` | 10.6 |
| `struct_dup____deadlock` | 28.4 |
| `struct_main` | 12.0 |
| `struct_multiple____deadlock` | 6.6 |
| `struct_nested` | 7.0 |
| `struct_nested____self_lock` | 7.0 |
| `struct_simple____partial_critical_section` | 27.2 |
| `struct_spin____lock_leak` | 5.3 |
| `unused_func` | 4.4 |
