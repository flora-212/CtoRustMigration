# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T12:07:00.640130
**Total Examples:** 62
**Passed (No UB):** 39
**Failed (UB Detected):** 23

## Summary

- **Clean Code Rate:** 39/62 (62.9%)
- **Total Time:** 120.9s
- **Average Time:** 2.0s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.3 |
| `array_const____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `P | 0.9 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 2.1 |
| `array_main____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `array_simple` | ❌ FAIL | ❌ error[E0308]: mismatched types | 1.0 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.3 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 24.5 |
| `global_assume2` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 2.3 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `global_assume____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `global_check` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 0.9 |
| `global_check____lock_leak` | ❌ FAIL | ❌ 13 \| pub type __pthread_list_t = __pthread_internal_list; | 4.4 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 4.2 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.3 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `P | 1.7 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.8 |
| `global_custom____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `global_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.7 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.0 |
| `global_read____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `global_rwlock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `R | 0.9 |
| `global_rwlock____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `global_while` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.3 |
| `global_while____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `struct_alias` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 1.0 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 1.0 |
| `struct_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `struct_assume____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.3 |
| `struct_condvar` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 1.0 |
| `struct_condvar____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.3 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 1.0 |
| `struct_empty` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 0.9 |
| `struct_init` | ❌ FAIL | ❌ error[E0614]: type `Ss` cannot be dereferenced | 1.6 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error: cast cannot be followed by a method call | 0.9 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 3.1 |
| `struct_malloc2` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 1.0 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `struct_malloc____lost_wakeup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.4 |
| `struct_multiple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.3 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `struct_nested` | ❌ FAIL | ❌ error[E0308]: mismatched types | 1.0 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `struct_simple____partial_critical_section` | ❌ FAIL | ❌ error[E0308]: mismatched types | 1.0 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 0.9 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.0 |
| `struct_timedwait` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.5 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0609]: no field `n1` on type `std::sync::MutexGuard | 1.1 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0382]: the type `Arc` does not implement `Copy` | 0.9 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 1.4 |
| `unused_func____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.0 |

## UB Detected (Failures)

### array_const____lock_mismatch

```
Message: ❌ error[E0015]: cannot call non-const associated function `PthreadMutex::new` in statics

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const____lock_mismatch_h72bv17u)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `Arc`
 --> src/main.rs:1:17
  |
1
```

### array_main____lock_leak

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main____lock_leak_ec0qm2md)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.95s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated function `std::
```

### array_simple

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_simple_7xlhwjzk)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0308]: mismatched types
   --> src/main.rs:59:13
    |
 58 |      
```

### global_assume2

```
Message: ❌ error[E0425]: cannot find function `pthread_mutex_lock` in this scope

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume2_p9su_43t)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.17s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0425]: cannot find function `pthread_mutex_lock` in this scope
 
```

### global_check

```
Message: ❌ error[E0425]: cannot find function `pthread_mutex_lock` in this scope

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check_b4ad3c10)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0425]: cannot find function `pthread_mutex_lock` in this scope
  -
```

### global_check____lock_leak

```
Message: ❌ 13 | pub type __pthread_list_t = __pthread_internal_list;

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_leak_pfi832ee)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.32s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `std::os::raw::c_void`
 --> src/main
```

### global_check____lock_mismatch

```
Message: ❌ error[E0425]: cannot find function `pthread_mutex_lock` in this scope

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_mismatch_75n3wk20)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.10s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0425]: cannot find function `pthread_mutex_lock` 
```

### global_condvar____lost_wakeup

```
Message: ❌ error[E0015]: cannot call non-const associated function `PthreadMutex::new` in statics

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_condvar____lost_wakeup_9oyg37et)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.64s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated function 
```

### global_rwlock

```
Message: ❌ error[E0015]: cannot call non-const associated function `RwLock::new` in statics

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock_wcbsa4cl)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `Arc`
 --> src/main.rs:1:17
  |
1 | use std::syn
```

### struct_alias

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<St>>::new

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias_k19xj69w)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.88s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated function `std::sync::Arc::
```

### struct_alias____self_lock

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<St>::new` in statics

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias____self_lock_0yx5yd_k)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated function `std
```

### struct_condvar

```
Message: ❌ error[E0594]: cannot assign to data in an `Arc`

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_condvar_5vxrh238)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.88s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0594]: cannot assign to data in an `Arc`
  --> src/main.rs:33:5

```

### struct_dup____deadlock

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<Ss1>::new` in statics

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_dup____deadlock_0a40pf73)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated function `std::s
```

### struct_empty

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<Ss>>::new

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_empty_4lhltbsk)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0015]: cannot call non-const associated function `std::sync::Arc::
```

### struct_init

```
Message: ❌ error[E0614]: type `Ss` cannot be dereferenced

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init_xnwameos)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.50s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0614]: type `Ss` cannot be dereferenced
  --> src/main.rs:29:19
   
```

### struct_init____partial_critical_section

```
Message: ❌ error: cast cannot be followed by a method call

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init____partial_critical_section_r9uf19_y)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error: cast cannot be followed by a method cal
```

### struct_malloc

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc_d14zdp13)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.96s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0308]: mismatched types
   --> src/main.rs:18:22
    |
 18 |     
```

### struct_malloc2

```
Message: ❌ error[E0594]: cannot assign to data in an `Arc`

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc2_4qi91jvb)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: variable does not need to be mutable
  --> src/main.rs:25:9
  
```

### struct_nested

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_nested_55gup5ul)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `std::ffi::CString`
 --> src/main.rs:3:5
  |
3 |
```

### struct_simple____partial_critical_section

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_simple____partial_critical_section_mgl_xcr3)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0308]: mismatched types
  --> src/mai
```

### struct_spin

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_spin_qpjhg12_)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.84s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0308]: mismatched types
   --> src/main.rs:40:31
    |
 40 |     le
```

### struct_timedwait____deadlock

```
Message: ❌ error[E0609]: no field `n1` on type `std::sync::MutexGuard<'_, ()>`

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____deadlock_7j2yl6ke)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.95s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `UNIX_EPOCH`
 --> src/main.rs:3:3
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0382]: the type `Arc` does not implement `Copy`

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____lost_wakeup_9t7wzd_q)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `c_void`
 --> src/main.rs:5:27
```

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const` | 1.5 |
| `array_const____deadlock` | 1.3 |
| `array_main` | 1.4 |
| `array_main____partial_critical_section` | 1.5 |
| `array_simple____partial_critical_section` | 1.3 |
| `global_assume` | 24.5 |
| `global_assume2____self_lock` | 2.7 |
| `global_assume____lock_leak` | 1.4 |
| `global_condvar` | 1.3 |
| `global_condvar____partial_critical_section` | 1.4 |
| `global_custom` | 2.8 |
| `global_custom____self_lock` | 1.4 |
| `global_main` | 3.7 |
| `global_main____self_lock` | 1.4 |
| `global_nested` | 1.7 |
| `global_nested____deadlock` | 1.4 |
| `global_read` | 2.0 |
| `global_read____lock_mismatch` | 1.5 |
| `global_rwlock____lock_leak` | 1.4 |
| `global_simple` | 1.5 |
| `global_simple____partial_critical_section` | 1.4 |
| `global_trylock` | 1.5 |
| `global_while` | 1.3 |
| `global_while____lock_leak` | 1.4 |
| `struct_assume` | 1.4 |
| `struct_assume____deadlock` | 1.3 |
| `struct_condvar____lost_wakeup` | 1.3 |
| `struct_dup` | 1.4 |
| `struct_main` | 2.1 |
| `struct_malloc2____lock_mismatch` | 1.5 |
| `struct_malloc____lost_wakeup` | 2.4 |
| `struct_multiple` | 1.3 |
| `struct_multiple____deadlock` | 1.4 |
| `struct_nested____self_lock` | 1.4 |
| `struct_simple` | 1.4 |
| `struct_spin____lock_leak` | 2.0 |
| `struct_timedwait` | 1.5 |
| `unused_func` | 1.4 |
| `unused_func____lock_mismatch` | 2.0 |
