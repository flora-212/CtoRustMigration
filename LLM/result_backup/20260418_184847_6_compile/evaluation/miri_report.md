# Miri Undefined Behavior Detection Report

**Generated:** 2026-04-21T12:55:54.183361
**Total Examples:** 62
**Passed (No UB):** 31
**Failed (UB Detected):** 31

## Summary

- **Clean Code Rate:** 31/62 (50.0%)
- **Total Time:** 490.2s
- **Average Time:** 7.9s per example

## Detailed Results

| Example | Status | Message | Time (s) |
|---------|--------|---------|----------|
| `array_const` | ❌ FAIL | ❌ error[E0599]: no method named `lock` found for struct `std | 13.0 |
| `array_const____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 5.0 |
| `array_const____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.4 |
| `array_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.5 |
| `array_main____lock_leak` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 6.7 |
| `array_main____partial_critical_section` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 3.3 |
| `array_simple` | ❌ FAIL | ❌ error[E0308]: mismatched types | 2.1 |
| `array_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.7 |
| `global_assume` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.6 |
| `global_assume2` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 7.7 |
| `global_assume2____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.2 |
| `global_assume____lock_leak` | ❌ FAIL | ❌ error[E0507]: cannot move out of static item `NUM_MUTEX` | 11.5 |
| `global_check` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.6 |
| `global_check____lock_leak` | ❌ FAIL | ❌ error[E0382]: use of moved value: `m` | 2.3 |
| `global_check____lock_mismatch` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 19.9 |
| `global_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.3 |
| `global_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `P | 1.4 |
| `global_condvar____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.7 |
| `global_custom` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `global_custom____self_lock` | ❌ FAIL | ❌ error[E0599]: no method named `unwrap` found for unit type | 9.5 |
| `global_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `global_main____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.0 |
| `global_nested` | ❌ FAIL | ❌ error[E0596]: cannot borrow `n1` as mutable, as it is not  | 4.3 |
| `global_nested____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.8 |
| `global_read` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.1 |
| `global_read____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `s | 7.5 |
| `global_rwlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.7 |
| `global_rwlock____lock_leak` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `R | 5.7 |
| `global_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `global_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `global_trylock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 7.1 |
| `global_while` | ❌ FAIL | ❌ error[E0425]: cannot find function `pthread_mutex_lock` in | 28.4 |
| `global_while____lock_leak` | ❌ FAIL | ❌ error[E0382]: the type `Arc` does not implement `Copy` | 18.4 |
| `struct_alias` | ❌ FAIL | ❌ error[E0502]: cannot borrow `*s` as mutable because it is  | 1.3 |
| `struct_alias____self_lock` | ❌ FAIL | ❌ error[E0596]: cannot borrow `s1` as mutable, as it is not  | 16.3 |
| `struct_assume` | ❌ FAIL | ❌ error[E0308]: mismatched types | 3.6 |
| `struct_assume____deadlock` | ❌ FAIL | ❌ Compiling miri_test v0.1.0 (/tmp/miri_test_struct_assume__ | 4.1 |
| `struct_condvar` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.1 |
| `struct_condvar____lost_wakeup` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 4.6 |
| `struct_dup` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 6.9 |
| `struct_dup____deadlock` | ❌ FAIL | ❌ error[E0382]: the type `Arc` does not implement `Copy` | 34.7 |
| `struct_empty` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.2 |
| `struct_init` | ❌ FAIL | ❌ error[E0658]: use of unstable library feature `get_mut_unc | 6.6 |
| `struct_init____partial_critical_section` | ❌ FAIL | ❌ error[E0382]: the type `Arc` does not implement `Copy` | 10.3 |
| `struct_main` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.2 |
| `struct_malloc` | ❌ FAIL | ❌ error[E0308]: mismatched types | 5.1 |
| `struct_malloc2` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 11.0 |
| `struct_malloc2____lock_mismatch` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.6 |
| `struct_malloc____lost_wakeup` | ❌ FAIL | ❌ error[E0594]: cannot assign to data in an `Arc` | 9.3 |
| `struct_multiple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 3.7 |
| `struct_multiple____deadlock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 12.6 |
| `struct_nested` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 13.1 |
| `struct_nested____self_lock` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 19.1 |
| `struct_simple` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 9.6 |
| `struct_simple____partial_critical_section` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 8.2 |
| `struct_spin` | ❌ FAIL | ❌ error[E0308]: mismatched types | 6.9 |
| `struct_spin____lock_leak` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 2.0 |
| `struct_timedwait` | ❌ FAIL | ❌ error[E0382]: use of moved value: `guard` | 4.9 |
| `struct_timedwait____deadlock` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 1.5 |
| `struct_timedwait____lost_wakeup` | ❌ FAIL | ❌ error[E0255]: the name `timespec` is defined multiple time | 6.4 |
| `unused_func` | ✅ PASS | ✅ Miri tests passed (no UB detected) | 31.9 |
| `unused_func____lock_mismatch` | ❌ FAIL | ❌ error[E0015]: cannot call non-const associated function `P | 10.0 |

## UB Detected (Failures)

### array_const

```
Message: ❌ error[E0599]: no method named `lock` found for struct `std::sync::Arc<MutexArray>` in the current sc

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_const_b6j_btoi)
    Finished `test` profile [unoptimized + debuginf
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
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_main____partial_critical_section_0qfrcjpu)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.01s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691
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
   Compiling miri_test v0.1.0 (/tmp/miri_test_array_simple_i1jfw0m3)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.88s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `Arc`
 --> src/main.rs:1:17
  |
1 | use std::sync
```

### global_assume2

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume2_e_4cmsfq)
    Finished `test` profile [unoptimized + debug
```

### global_assume____lock_leak

```
Message: ❌ error[E0507]: cannot move out of static item `NUM_MUTEX`

Details:
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_assume____lock_leak_8tosmekc)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 11.23s
     Running u
```

### global_check____lock_leak

```
Message: ❌ error[E0382]: use of moved value: `m`

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_leak_73zer5fp)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.94s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0382]: use of moved value: `m`
  --> src/main.rs:49:1
```

### global_check____lock_mismatch

```
Message: ❌ error[E0425]: cannot find function `pthread_mutex_lock` in this scope

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_check____lock_mismatch_u_tbfi3u)
    Finished `test` profile [unop
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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_condvar____lost_wakeup_zpba7zfi)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.26s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
warning: unused import: `Arc`
 --> src/main.rs:1:17
  |

```

### global_custom____self_lock

```
Message: ❌ error[E0599]: no method named `unwrap` found for unit type `()` in the current scope

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_custom____self
```

### global_nested

```
Message: ❌ error[E0596]: cannot borrow `n1` as mutable, as it is not declared as mutable

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_nested_8kss0s7
```

### global_read____lock_mismatch

```
Message: ❌ error[E0015]: cannot call non-const associated function `std::sync::Arc::<std::sync::Mutex<i32>>::ne

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_read____lock_mismatch_rg_fmix0)
    Finished `test` profile [unopt
```

### global_rwlock____lock_leak

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_rwlock____lock
```

### global_while

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_while_k0oxaeu9
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
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_global_while____lock_
```

### struct_alias

```
Message: ❌ error[E0502]: cannot borrow `*s` as mutable because it is also borrowed as immutable

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias_nx8io1z4)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.04s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0502]: cannot borrow `*s` as mutable because it is also borrowed a
```

### struct_alias____self_lock

```
Message: ❌ error[E0596]: cannot borrow `s1` as mutable, as it is not declared as mutable

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_alias____self_
```

### struct_assume

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_assume_epshx3yp)
    Finished `test` profile [unoptimized + debugi
```

### struct_assume____deadlock

```
Message: ❌ Compiling miri_test v0.1.0 (/tmp/miri_test_struct_assume____deadlock_iusub5it)

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_assume____deadlock_iusub5it)
    Finished `test` profile [unoptimi
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
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_condvar____los
```

### struct_dup____deadlock

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_dup____deadloc
```

### struct_init

```
Message: ❌ error[E0658]: use of unstable library feature `get_mut_unchecked`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init_1n_7i2ba)
```

### struct_init____partial_critical_section

```
Message: ❌ error[E0382]: the type `Arc` does not implement `Copy`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_init____partial_critical_section_csnajtmx)
    Finished `test` pro
```

### struct_malloc

```
Message: ❌ error[E0308]: mismatched types

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling libc v0.2.185
   Compiling lazy_static v1.5.0
   Compiling once_cell v1.21.4
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc__zv0abp7)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.86s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/d
```

### struct_malloc2

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc2_xfth3le7)
    Finished `test` profile [unoptimized + debug
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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_malloc____lost_wakeup_3r71e94g)
    Finished `test` profile [unopt
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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_spin_cx658qos)
    Finished `test` profile [unoptimized + debuginf
```

### struct_timedwait

```
Message: ❌ error[E0382]: use of moved value: `guard`

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait_mcqt
```

### struct_timedwait____deadlock

```
Message: ❌ error[E0255]: the name `timespec` is defined multiple times

Details:
    Updating crates.io index
     Locking 3 packages to latest compatible versions
   Compiling libc v0.2.185
   Compiling once_cell v1.21.4
   Compiling lazy_static v1.5.0
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____deadlock_eojg9w77)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.30s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/miri_test-d1ce1253d7691017)
error[E0255]: the name `timespec` is defined multiple tim
```

### struct_timedwait____lost_wakeup

```
Message: ❌ error[E0255]: the name `timespec` is defined multiple times

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_struct_timedwait____lost_wakeup__zhi2ipn)
    Finished `test` profile [un
```

### unused_func____lock_mismatch

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
   Compiling miri_test v0.1.0 (/tmp/miri_test_unused_func____lock_m
```

## Safe Examples (No UB)

| Example | Time (s) |
|---------|----------|
| `array_const____deadlock` | 5.0 |
| `array_const____lock_mismatch` | 9.4 |
| `array_main` | 3.5 |
| `array_simple____partial_critical_section` | 7.7 |
| `global_assume` | 3.6 |
| `global_assume2____self_lock` | 3.2 |
| `global_check` | 2.6 |
| `global_condvar` | 3.3 |
| `global_condvar____partial_critical_section` | 9.7 |
| `global_custom` | 2.7 |
| `global_main` | 2.1 |
| `global_main____self_lock` | 3.0 |
| `global_nested____deadlock` | 2.8 |
| `global_read` | 7.1 |
| `global_rwlock` | 2.7 |
| `global_simple` | 2.1 |
| `global_simple____partial_critical_section` | 2.1 |
| `global_trylock` | 7.1 |
| `struct_condvar` | 2.1 |
| `struct_dup` | 6.9 |
| `struct_empty` | 2.2 |
| `struct_main` | 8.2 |
| `struct_malloc2____lock_mismatch` | 12.6 |
| `struct_multiple` | 3.7 |
| `struct_multiple____deadlock` | 12.6 |
| `struct_nested` | 13.1 |
| `struct_nested____self_lock` | 19.1 |
| `struct_simple` | 9.6 |
| `struct_simple____partial_critical_section` | 8.2 |
| `struct_spin____lock_leak` | 2.0 |
| `unused_func` | 31.9 |
