use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
    pub __flags: u32,
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
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [i8; 56],
    pub __align: i64,
}

pub type C2Rust_Unnamed = u32;

pub const PTHREAD_RWLOCK_DEFAULT_NP: C2Rust_Unnamed = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: C2Rust_Unnamed = 0;

pub const NULL: *mut c_void = ptr::null_mut();

#[no_mangle]
pub static mut n: c_int = 0;

#[no_mangle]
pub static mut lock: pthread_rwlock_t = pthread_rwlock_t {
    __data: __pthread_rwlock_arch_t {
        __readers: 0,
        __writers: 0,
        __wrphase_futex: 0,
        __writers_futex: 0,
        __pad3: 0,
        __pad4: 0,
        __cur_writer: 0,
        __shared: 0,
        __rwelision: 0,
        __pad1: [0; 7],
        __pad2: 0,
        __flags: PTHREAD_RWLOCK_DEFAULT_NP,
    },
};

#[no_mangle]
pub unsafe extern "C" fn f1() -> c_int {
    let mut x: c_int = 0;
    pthread_rwlock_rdlock(&raw mut lock);
    x = n;
    pthread_rwlock_unlock(&raw mut lock);
    x
}

#[no_mangle]
pub unsafe extern "C" fn f2() {
    pthread_rwlock_wrlock(&raw mut lock);
    n += 1;
    pthread_rwlock_unlock(&raw mut lock);
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    NULL
}

unsafe fn main_0() -> c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;

    pthread_create(
        &raw mut id1,
        ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
        NULL,
    );

    pthread_create(
        &raw mut id2,
        ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
        NULL,
    );

    pthread_join(id1, ptr::null_mut::<*mut c_void>());
    pthread_join(id2, ptr::null_mut::<*mut c_void>());

    let c_string = CString::new(format!("{}\n", n)).unwrap();
    printf(c_string.as_ptr());

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}