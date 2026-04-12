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
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed = 0;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;

pub struct mData {
    pub n: ::core::ffi::c_int,
}
pub static mut m: Mutex<mData> = Mutex::new(mData {
    n: 0 as ::core::ffi::c_int,
});
#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut m_guard_opt: Option<std::sync::MutexGuard<'_, _>> = None;
    let mut m_guard; // Will be assigned by lock/trylock

    let mut x: ::core::ffi::c_int = 0;
    m_guard = m.lock().unwrap();
    (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
    drop(m_guard);
    x = {
        m_guard_opt = m.lock().ok();
        if m_guard_opt.is_some() {
            0
        } else {
            libc::ENOTRECOVERABLE
        }
    };
    match m_guard_opt {
        None => return,
        Some(m_guard_tmp) => {
            m_guard = m_guard_tmp;
            (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
            drop(m_guard);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn f2() {
    let mut m_guard_opt: Option<std::sync::MutexGuard<'_, _>> = None;
    let mut m_guard; // Will be assigned by lock/trylock

    let mut x: ::core::ffi::c_int = 0;
    m_guard = m.lock().unwrap();
    (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
    drop(m_guard);
    x = {
        m_guard_opt = m.try_lock().ok();
        if m_guard_opt.is_some() {
            0
        } else {
            libc::EBUSY
        }
    };
    match m_guard_opt {
        None => return,
        Some(m_guard_tmp) => {
            m_guard = m_guard_tmp;
            (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
            drop(m_guard);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn f3() {
    let mut m_guard_opt: Option<std::sync::MutexGuard<'_, _>> = None;
    let mut m_guard; // Will be assigned by lock/trylock

    let mut x: ::core::ffi::c_int = 0;
    m_guard = m.lock().unwrap();
    (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
    drop(m_guard);
    x = {
        m_guard_opt = m.lock().ok();
        if m_guard_opt.is_some() {
            0
        } else {
            libc::ENOTRECOVERABLE
        }
    };
    match m_guard_opt {
        None => return,
        Some(m_guard_tmp) => {
            m_guard = m_guard_tmp;
            (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
            drop(m_guard);
        }
    }
    x = {
        m_guard_opt = m.try_lock().ok();
        if m_guard_opt.is_some() {
            0
        } else {
            libc::EBUSY
        }
    };
    match m_guard_opt {
        None => return,
        Some(m_guard_tmp) => {
            m_guard = m_guard_tmp;
            (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
            drop(m_guard);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn f4() {
    let mut m_guard_opt: Option<std::sync::MutexGuard<'_, _>> = None;
    let mut m_guard; // Will be assigned by lock/trylock

    let mut x: ::core::ffi::c_int = 0;
    m_guard = m.lock().unwrap();
    (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
    drop(m_guard);
    x = {
        m_guard_opt = m.try_lock().ok();
        if m_guard_opt.is_some() {
            0
        } else {
            libc::EBUSY
        }
    };
    match m_guard_opt {
        None => return,
        Some(m_guard_tmp) => {
            m_guard = m_guard_tmp;
            (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
            drop(m_guard);
        }
    }
    x = {
        m_guard_opt = m.lock().ok();
        if m_guard_opt.is_some() {
            0
        } else {
            libc::ENOTRECOVERABLE
        }
    };
    match m_guard_opt {
        None => return,
        Some(m_guard_tmp) => {
            m_guard = m_guard_tmp;
            (*m_guard).n = (*m_guard).n + 1 as ::core::ffi::c_int;
            drop(m_guard);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1();
    f2();
    f3();
    f4();
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
        m.get_mut().unwrap().n,
    );
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
