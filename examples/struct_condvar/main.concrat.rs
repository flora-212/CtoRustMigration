use std::{
    sync::{Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::Duration,
};
extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed_0 = 0;

#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub m1: Mutex<C2Rust_Unnamed_1m1Data>,
    pub cond: Condvar,
}
pub struct C2Rust_Unnamed_1m1Data {
    pub n1: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub static mut s: C2Rust_Unnamed_1 = C2Rust_Unnamed_1 {
    m1: Mutex::new(C2Rust_Unnamed_1m1Data {
        n1: 0 as ::core::ffi::c_int,
    }),
    cond: Condvar::new(),
};
#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut s_m1_guard; // Will be assigned by lock/trylock

    s_m1_guard = s.m1.lock().unwrap();
    (*s_m1_guard).n1 = (*s_m1_guard).n1 + 1 as ::core::ffi::c_int;
    if (*s_m1_guard).n1 == 1 as ::core::ffi::c_int {
        {
            s_m1_guard = s.cond.wait(s_m1_guard).unwrap();
            0
        };
    } else {
        s.cond.notify_one();
    }
    drop(s_m1_guard);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1();
    return NULL;
}
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &raw mut id1,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        NULL,
    );
    pthread_create(
        &raw mut id2,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        NULL,
    );
    pthread_join(id1, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_join(id2, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    printf(
        b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        s.m1.get_mut().unwrap().n1,
    );
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
