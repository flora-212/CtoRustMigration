use std::sync::{Arc, Mutex};
use std::thread;

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
pub static mut n: i32 = 0;

#[no_mangle]
pub static mut m: pthread_mutex_t = pthread_mutex_t {
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
};

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut x: i32 = 0;
    pthread_mutex_lock(&mut m);
    n = n + 1;
    pthread_mutex_unlock(&mut m);
    x = pthread_mutex_lock(&mut m);
    if x != 0 {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&mut m);
    }
}

#[no_mangle]
pub unsafe extern "C" fn f2() {
    let mut x: i32 = 0;
    pthread_mutex_lock(&mut m);
    n = n + 1;
    pthread_mutex_unlock(&mut m);
    x = pthread_mutex_trylock(&mut m);
    if x != 0 {
        return;
    } else {
        n = n + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn f3() {
    let mut x: i32 = 0;
    pthread_mutex_lock(&mut m);
    n = n + 1;
    x = pthread_mutex_lock(&mut m);
    if x != 0 {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&mut m);
    }
    x = pthread_mutex_trylock(&mut m);
    if x != 0 {
        return;
    } else {
        n = n + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn f4() {
    let mut x: i32 = 0;
    pthread_mutex_lock(&mut m);
    n = n + 1;
    x = pthread_mutex_trylock(&mut m);
    if x != 0 {
        return;
    } else {
        n = n + 1;
    }
    x = pthread_mutex_lock(&mut m);
    if x != 0 {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&mut m);
    }
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut i8) -> *mut i8 {
    f1();
    f2();
    f3();
    f4();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let mut id1: u64 = 0;
    let mut id2: u64 = 0;
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
    println!("{}", n);
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}