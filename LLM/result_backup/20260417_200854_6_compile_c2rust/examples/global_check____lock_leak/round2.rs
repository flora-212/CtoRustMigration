use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

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

pub const NULL: *mut i8 = ptr::null_mut();

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
            __prev: ptr::null_mut(),
            __next: ptr::null_mut(),
        },
    },
};

#[no_mangle]
pub unsafe extern "C" fn f1(mutex: *mut pthread_mutex_t) {
    let mut x: i32 = 0;
    pthread_mutex_lock(mutex);
    n = n + 1;
    pthread_mutex_unlock(mutex);
    x = pthread_mutex_lock(mutex);
    if x != 0 {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(mutex);
    }
}

#[no_mangle]
pub unsafe extern "C" fn f2(mutex: *mut pthread_mutex_t) {
    let mut x: i32 = 0;
    pthread_mutex_lock(mutex);
    n = n + 1;
    pthread_mutex_unlock(mutex);
    x = pthread_mutex_trylock(mutex);
    if x != 0 {
        return;
    } else {
        n = n + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn f3(mutex: *mut pthread_mutex_t) {
    let mut x: i32 = 0;
    pthread_mutex_lock(mutex);
    n = n + 1;
    x = pthread_mutex_lock(mutex);
    if x != 0 {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(mutex);
    }
    x = pthread_mutex_trylock(mutex);
    if x != 0 {
        return;
    } else {
        n = n + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn f4(mutex: *mut pthread_mutex_t) {
    let mut x: i32 = 0;
    pthread_mutex_lock(mutex);
    n = n + 1;
    x = pthread_mutex_trylock(mutex);
    if x != 0 {
        return;
    } else {
        n = n + 1;
    }
    x = pthread_mutex_lock(mutex);
    if x != 0 {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(mutex);
    }
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut i8, mutex: *mut pthread_mutex_t) -> *mut i8 {
    f1(mutex);
    f2(mutex);
    f3(mutex);
    f4(mutex);
    return NULL;
}

unsafe fn main_0() -> i32 {
    let mutex = Arc::new(Mutex::new(()));
    let mutex_clone = Arc::clone(&mutex);

    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut(), Arc::as_ptr(&mutex_clone) as *mut pthread_mutex_t);
    });

    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut(), Arc::as_ptr(&mutex_clone) as *mut pthread_mutex_t);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n_value = *mutex.lock().unwrap();
    let c_string = CString::new(format!("{}\n", n_value)).unwrap();
    libc::printf(c_string.as_ptr());

    return 0;
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}