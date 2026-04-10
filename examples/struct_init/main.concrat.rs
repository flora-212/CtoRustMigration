use std::{
    sync::{Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::Duration,
};
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
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

#[repr(C)]
pub struct ss {
    pub m: Mutex<ssmData>,
}
pub struct ssmData {
    pub n: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub static mut s1: ss = ss {
    m: Mutex::new(ssmData { n: 0 }),
};
#[no_mangle]
pub static mut s2: ss = ss {
    m: Mutex::new(ssmData { n: 0 }),
};
#[no_mangle]
pub unsafe extern "C" fn f1(mut s: *mut ss) {
    let mut s_m_guard; // Will be assigned by lock/trylock

    s_m_guard = (*s).m.lock().unwrap();
    (*s_m_guard).n = (*s_m_guard).n + 1 as ::core::ffi::c_int;
    drop(s_m_guard);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1(&raw mut s1);
    f1(&raw mut s2);
    f1(arg as *mut ss);
    return NULL;
}
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut s3: *mut ss = ::core::ptr::null_mut::<ss>();
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    0;
    ();
    0;
    s3 = malloc(::core::mem::size_of::<ss>() as size_t) as *mut ss;
    ();
    (*s3).m = Mutex::new(ssmData {
        n: 0 as ::core::ffi::c_int,
    });
    pthread_create(
        &raw mut id1,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        s3 as *mut ::core::ffi::c_void,
    );
    pthread_create(
        &raw mut id2,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        s3 as *mut ::core::ffi::c_void,
    );
    pthread_join(id1, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_join(id2, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    printf(
        b"%d %d %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        s1.m.get_mut().unwrap().n,
        s2.m.get_mut().unwrap().n,
        (*s3).m.get_mut().unwrap().n,
    );
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
