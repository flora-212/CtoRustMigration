# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T13:04:41.870618
**Total Examples:** 62
**Passed (No UB):** 36
**Failed (UB Detected):** 26

## Summary

- **Clean Code Rate:** 36/62 (58.1%)
- **Total Time:** 358.5s
- **Average Time:** 5.8s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ error[E0277]: the trait bound `std::sync::Mutex<i32>: std: | 23.4 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.4 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `P | 4.0 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.4 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 6.9 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 3.7 |
| `array_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |
| `array_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 4.9 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |
| `global_assume2` | ❌ FAIL | ❌ error[E0594]: cannot assign to `NUM_MUTEX.value`, as `NUM_ | 3.9 |
| `global_assume2____self_lock` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 4.5 |
| `global_assume____lock_leak` | ❌ FAIL | ❌ error[E0507]: cannot move out of static item `NUM_MUTEX` | 1.1 |
| `global_check` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `global_check____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.6 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 4.9 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 24.9 |
| `global_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.1 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.8 |
| `global_custom____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.5 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.3 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.4 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.9 |
| `global_read____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.1 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `R | 49.7 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `R | 0.9 |
| `global_simple` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `std | 1.4 |
| `global_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 2.7 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.9 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error[E0382]: the type `Arc` does not implement `Copy` | 1.3 |
| `struct_alias` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.8 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 1.1 |
| `struct_assume` | ❌ FAIL | ❌ error[E0308]: mismatched types | 1.1 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for reference ` | 1.3 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 1.0 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 1.2 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.0 |
| `struct_dup____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `struct_empty` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 14.7 |
| `struct_init` | ❌ FAIL | ❌ error[E0308]: mismatched types | 2.3 |
| `struct_init____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.9 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.0 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 18.6 |
| `struct_malloc2` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.2 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0308]: mismatched types | 6.2 |
| `struct_multiple` | ❌ FAIL | ❌ error[E0432]: unresolved import `std::sync::Lazy` | 20.9 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 26.4 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.4 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.6 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `struct_spin` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.0 |
| `struct_spin____lock_leak` | ❌ FAIL | ❌ error[E0308]: mismatched types | 2.2 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0599]: no method named `wait_until` found for struc | 2.9 |
| `struct_timedwait____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 4.0 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0599]: no method named `wait_until` found for struc | 1.9 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `unused_func____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.6 |

## UB Detected (Failures)

### array_const

```
Message: ❌ error[E0277]: the trait bound `std::sync::Mutex<i32>: std::marker::Copy` is not satisfied

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const_ai1xg70a)
```

### array_const____lock_mismatch

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const____lock_m
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
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main____lock_le
```

### array_main____partial_critical_section

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main____partial
```

### array_simple____partial_critical_section

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_simple____parti
```

### global_assume2

```
Message: ❌ error[E0594]: cannot assign to `NUM_MUTEX.value`, as `NUM_MUTEX` is an immutable static item

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume2_suiru6
```

### global_assume2____self_lock

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume2____sel
```

### global_assume____lock_leak

```
Message: ❌ error[E0507]: cannot move out of static item `NUM_MUTEX`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume____lock_leak_rke0v9nu)
    Finished `test` profile [unoptim
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
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_mismatch_lnib4y_n)
    Finished `test` profile [unop
```

### global_rwlock

```
Message: ❌ error[E0015]: cannot call non-const associated function `RwLock::new` in statics

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock_1urw6iu
```

### global_rwlock____lock_leak

```
Message: ❌ error[E0015]: cannot call non-const associated function `RwLock::new` in statics

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock____lock_leak_hutj_v7_)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `Arc`
 --> src/main.rs:1:17
  |
1 |
```

### global_simple

```
Message: ❌ error[E0599]: no method named `lock` found for struct `std::sync::Arc<SharedData>` in the current sc

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_simple_vzlscqh2)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.27s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0599]: no method named `lock` found for struct `std::sync::Arc<Sh
```

### global_simple____partial_critical_section

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_simple____partial_critical_section_wi5ncnle)
    Finished `test` p
```

### global_while____lock_leak

```
Message: ❌ error[E0382]: the type `Arc` does not implement `Copy`

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_while____lock_leak_20gqjqq5)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.17s
     Running uni
```

### struct_alias____self_lock

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<St>::new` in statics

Details:
    Blocking waiting for file lock on package cache
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias____self_lock_4jdodwsp)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.01s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0
```

### struct_assume

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_assume_ab43brrb)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0308]: mismatched types
  --> src/main.rs:42:19
   |
42 |        
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
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_assume____deadlock_ii4h4ryg)
    Finished `test` profile [u
```

### struct_condvar

```
Message: ❌ error[E0594]: cannot assign to data in an `Arc`

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_condvar_6pxawcxf)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.88s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `std::ffi::CStr`
 --> src/main.rs:4:5
  |
4 | u
```

### struct_condvar____lost_wakeup

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_condvar____lost_wakeup_4e67ja82)
    Finished `test` profile [unop
```

### struct_init

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init_9v6_kot1)
```

### struct_malloc

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
    Blocking waiting for file lock on shared package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compi
```

### struct_malloc____lost_wakeup

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc____lost
```

### struct_multiple

```
Message: ❌ error[E0432]: unresolved import `std::sync::Lazy`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_multiple_aj_nq
```

### struct_spin____lock_leak

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_spin____lock_l
```

### struct_timedwait

```
Message: ❌ error[E0599]: no method named `wait_until` found for struct `std::sync::Condvar` in the current scop

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait_wxxw
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0599]: no method named `wait_until` found for struct `std::sync::Condvar` in the current scop

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____l
```

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const____deadlock` | 6.4 |
| `array_main` | 5.4 |
| `array_simple` | 4.9 |
| `global_assume` | 4.9 |
| `global_check` | 3.7 |
| `global_check____lock_leak` | 4.6 |
| `global_condvar` | 24.9 |
| `global_condvar____lost_wakeup` | 3.5 |
| `global_condvar____partial_critical_section` | 4.1 |
| `global_custom` | 3.8 |
| `global_custom____self_lock` | 2.5 |
| `global_main` | 3.2 |
| `global_main____self_lock` | 2.3 |
| `global_nested` | 3.4 |
| `global_nested____deadlock` | 3.2 |
| `global_read` | 2.9 |
| `global_read____lock_mismatch` | 3.1 |
| `global_trylock` | 1.9 |
| `global_while` | 1.5 |
| `struct_alias` | 1.8 |
| `struct_dup` | 2.0 |
| `struct_dup____deadlock` | 1.4 |
| `struct_empty` | 14.7 |
| `struct_init____partial_critical_section` | 4.9 |
| `struct_main` | 4.0 |
| `struct_malloc2` | 3.5 |
| `struct_malloc2____lock_mismatch` | 4.2 |
| `struct_multiple____deadlock` | 26.4 |
| `struct_nested` | 5.4 |
| `struct_nested____self_lock` | 3.6 |
| `struct_simple` | 2.6 |
| `struct_simple____partial_critical_section` | 2.7 |
| `struct_spin` | 3.0 |
| `struct_timedwait____deadlock` | 4.0 |
| `unused_func` | 3.5 |
| `unused_func____lock_mismatch` | 6.6 |
