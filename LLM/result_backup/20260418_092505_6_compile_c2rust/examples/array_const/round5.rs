use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;

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
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_list_t,
}

pub type pthread_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}

pub type C2Rust_Unnamed = u32;

pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed = 0;

#[no_mangle]
pub static mut n1: [i32; 3] = [0; 3];

#[no_mangle]
pub static mut num_mutex: [pthread_mutex_t; 3] = [
    pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: PTHREAD_MUTEX_TIMED_NP as i32,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: std::ptr::null_mut(),
                __next: std::ptr::null_mut(),
            },
        },
    },
    pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: PTHREAD_MUTEX_TIMED_NP as i32,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: std::ptr::null_mut(),
                __next: std::ptr::null_mut(),
            },
        },
    },
    pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: PTHREAD_MUTEX_TIMED_NP as i32,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: std::ptr::null_mut(),
                __next: std::ptr::null_mut(),
            },
        },
    },
];

#[no_mangle]
pub unsafe extern "C" fn f1() {
    pthread_mutex_lock(
        (&mut num_mutex as *mut pthread_mutex_t).offset(0) as *mut pthread_mutex_t,
    );
    n1[0] += 1;
    pthread_mutex_unlock(
        (&mut num_mutex as *mut pthread_mutex_t).offset(0) as *mut pthread_mutex_t,
    );
    pthread_mutex_lock(
        (&mut num_mutex as *mut pthread_mutex_t).offset(1) as *mut pthread_mutex_t,
    );
    n1[1] += 1;
    pthread_mutex_unlock(
        (&mut num_mutex as *mut pthread_mutex_t).offset(1) as *mut pthread_mutex_t,
    );
    pthread_mutex_lock(
        (&mut num_mutex as *mut pthread_mutex_t).offset(2) as *mut pthread_mutex_t,
    );
    n1[2] += 1;
    pthread_mutex_unlock(
        (&mut num_mutex as *mut pthread_mutex_t).offset(2) as *mut pthread_mutex_t,
    );
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut i8) -> *mut i8 {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun as unsafe extern "C" fn(*mut i8) -> *mut i8),
        std::ptr::null_mut(),
    );
    pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun as unsafe extern "C" fn(*mut i8) -> *mut i8),
        std::ptr::null_mut(),
    );
    pthread_join(id1, std::ptr::null_mut());
    pthread_join(id2, std::ptr::null_mut());
    printf(
        b"%d %d %d\n\0".as_ptr() as *const i8,
        n1[0],
        n1[1],
        n1[2],
    );
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}