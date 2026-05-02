# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T13:00:57.745307
**Total Examples:** 62
**Passed (No UB):** 29
**Failed (UB Detected):** 33

## Summary

- **Clean Code Rate:** 29/62 (46.8%)
- **Total Time:** 543.9s
- **Average Time:** 8.8s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 3.6 |
| `array_const____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `std | 6.1 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `std | 1.3 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 35.0 |
| `array_main____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.2 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 1.4 |
| `array_simple` | ❌ FAIL | ❌ error[E0277]: the trait bound `std::sync::Arc<std::sync::M | 8.2 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.5 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.7 |
| `global_assume2` | ❌ FAIL | ❌ error[E0308]: mismatched types | 3.4 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 10.8 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.9 |
| `global_check` | ❌ FAIL | ❌ error[E0596]: cannot borrow data in an `Arc` as mutable | 9.7 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 2.6 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 1.6 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.9 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.1 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 18.9 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.6 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0277]: `*mut libc::c_void` cannot be sent between t | 7.6 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.5 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.3 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.3 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 31.0 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0530]: let bindings cannot shadow statics | 9.6 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 28.6 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0530]: let bindings cannot shadow statics | 13.8 |
| `global_simple` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 8.6 |
| `global_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 9.5 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 11.9 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.4 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 34.0 |
| `struct_alias` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 8.6 |
| `struct_alias____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.1 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for reference ` | 5.4 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.6 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.3 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.1 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error[E0308]: mismatched types | 25.9 |
| `struct_empty` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 5.8 |
| `struct_init` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 4.6 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error[E0133]: use of mutable static is unsafe and requires | 5.1 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.1 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 2.7 |
| `struct_malloc2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `struct_malloc2____lock_mismatch` | ❌ FAIL | ❌ error[E0505]: cannot move out of `s` because it is borrowe | 4.9 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 4.5 |
| `struct_multiple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.2 |
| `struct_nested` | ❌ FAIL | ❌ error[E0204]: the trait `std::marker::Copy` cannot be impl | 3.8 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.8 |
| `struct_simple` | ❌ FAIL | ❌ error[E0382]: the type `Arc` does not implement `Copy` | 4.5 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 24.5 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 3.2 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.6 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0599]: no method named `wait_until` found for struc | 3.3 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 2.9 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 2.8 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.1 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error[E0716]: temporary value dropped while borrowed | 2.5 |

## UB Detected (Failures)

### array_const

```
Message: ❌ error[E0425]: cannot find function `pthread_mutex_lock` in this scope

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const_7gtp__76)
```

### array_const____deadlock

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
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const____deadlo
```

### array_const____lock_mismatch

```
Message: ❌ error[E0599]: no method named `lock` found for struct `std::sync::Arc<MutexArray>` in the current sc

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const____lock_mismatch_87afjq2f)
    Finished `test` profile [unopt
```

### array_main____partial_critical_section

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main____partial_critical_section_ct8z_52n)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.25s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691
```

### array_simple

```
Message: ❌ error[E0277]: the trait bound `std::sync::Arc<std::sync::Mutex<()>>: std::marker::Copy` is not satis

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_simple_4lg1j_6g)
    Finished `test` profile [unoptimized + debugin
```

### global_assume2

```
Message: ❌ error[E0308]: mismatched types

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume2_vv5aapmf)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.25s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/
```

### global_assume2____self_lock

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume2____sel
```

### global_check

```
Message: ❌ error[E0596]: cannot borrow data in an `Arc` as mutable

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check_k58y605y
```

### global_check____lock_leak

```
Message: ❌ error[E0425]: cannot find function `pthread_mutex_lock` in this scope

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_leak_wcyn1k1z)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.21s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux
```

### global_check____lock_mismatch

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_mismatch_qnu1hlvo)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.33s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated function 
```

### global_custom____self_lock

```
Message: ❌ error[E0277]: `*mut libc::c_void` cannot be sent between threads safely

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

### global_read____lock_mismatch

```
Message: ❌ error[E0530]: let bindings cannot shadow statics

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_read____lock_m
```

### global_rwlock

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<RwLock>::new` in statics

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock__vjyycf
```

### global_rwlock____lock_leak

```
Message: ❌ error[E0530]: let bindings cannot shadow statics

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock____lock
```

### global_simple

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_simple_44_9uj_
```

### global_simple____partial_critical_section

```
Message: ❌ error[E0308]: mismatched types

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_simple____partial_critical_section_wpzm9mns)
    Finished `test` p
```

### global_while____lock_leak

```
Message: ❌ error[E0425]: cannot find function `pthread_mutex_lock` in this scope

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_while____lock_
```

### struct_alias

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<St>>::new

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias_it4iv_g1
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

### struct_dup____deadlock

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
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_dup____deadlock_y3l7852_)
    Finished `test` profile [unoptimized
```

### struct_empty

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<Ss>>::new

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_empty_d7bmvsnb)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.67s
     Running unittests src/ma
```

### struct_init

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<Ss>>::new

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init_jb9lit0_)
    Finished `test` profile [unoptimized + debuginf
```

### struct_init____partial_critical_section

```
Message: ❌ error[E0133]: use of mutable static is unsafe and requires unsafe function or block

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init____partial_critical_section_lzbhe9m_)
    Finished `test` pro
```

### struct_malloc

```
Message: ❌ error[E0594]: cannot assign to data in an `Arc`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc_9lxi1gw
```

### struct_malloc2____lock_mismatch

```
Message: ❌ error[E0505]: cannot move out of `s` because it is borrowed

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc2____loc
```

### struct_malloc____lost_wakeup

```
Message: ❌ error[E0594]: cannot assign to data in an `Arc`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc____lost_wakeup_0jhtiry_)
    Finished `test` profile [unopt
```

### struct_nested

```
Message: ❌ error[E0204]: the trait `std::marker::Copy` cannot be implemented for this type

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_nested__wdtserr)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.67s
     Running unittests src/m
```

### struct_simple

```
Message: ❌ error[E0382]: the type `Arc` does not implement `Copy`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_simple_90id0lh0)
    Finished `test` profile [unoptimized + debugi
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
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_spin_w1sph3ke)
```

### struct_timedwait

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait_vzsrhgdg)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.17s
     Running unittests sr
```

### struct_timedwait____deadlock

```
Message: ❌ error[E0255]: the name `timespec` is defined multiple times

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____deadlock_a9qt2tys)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.76s
     Running 
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0255]: the name `timespec` is defined multiple times

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwa
```

### unused_func____lock_mismatch

```
Message: ❌ error[E0716]: temporary value dropped while borrowed

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_unused_func____lock_mismatch_7e11xv_t)
    Finished `test` profile [unopt
```

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_main` | 35.0 |
| `array_main____lock_leak` | 8.2 |
| `array_simple____partial_critical_section` | 5.5 |
| `global_assume` | 4.7 |
| `global_assume____lock_leak` | 12.9 |
| `global_condvar` | 11.9 |
| `global_condvar____lost_wakeup` | 13.1 |
| `global_condvar____partial_critical_section` | 18.9 |
| `global_custom` | 9.6 |
| `global_main` | 8.5 |
| `global_main____self_lock` | 5.3 |
| `global_nested` | 2.4 |
| `global_nested____deadlock` | 7.3 |
| `global_read` | 31.0 |
| `global_trylock` | 11.9 |
| `global_while` | 12.4 |
| `struct_alias____self_lock` | 9.1 |
| `struct_assume` | 3.6 |
| `struct_condvar` | 7.6 |
| `struct_condvar____lost_wakeup` | 7.3 |
| `struct_dup` | 6.1 |
| `struct_main` | 8.1 |
| `struct_malloc2` | 5.0 |
| `struct_multiple` | 5.0 |
| `struct_multiple____deadlock` | 4.2 |
| `struct_nested____self_lock` | 5.8 |
| `struct_simple____partial_critical_section` | 24.5 |
| `struct_spin____lock_leak` | 4.6 |
| `unused_func` | 3.1 |
