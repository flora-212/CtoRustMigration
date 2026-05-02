use std::sync::{Arc, Mutex};
use std::thread;

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

#[no_mangle]
pub static mut n: i32 = 0;

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
pub unsafe extern "C" fn f1() -> i32 {
    let mut x: i32 = 0;
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
pub unsafe extern "C" fn t_fun(mut arg: *mut i8) -> *mut i8 {
    f1();
    f2();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &raw mut id1,
        std::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut i8) -> *mut i8),
        std::ptr::null_mut(),
    );
    pthread_create(
        &raw mut id2,
        std::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut i8) -> *mut i8),
        std::ptr::null_mut(),
    );
    pthread_join(id1, std::ptr::null_mut::<*mut i8>());
    pthread_join(id2, std::ptr::null_mut::<*mut i8>());
    printf(b"%d\n\0".as_ptr() as *const i8, n);
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}