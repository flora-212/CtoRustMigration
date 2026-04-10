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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> ::core::ffi::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __rwelision: ::core::ffi::c_schar,
    pub __pad1: [::core::ffi::c_uchar; 7],
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlockattr_t {
    pub __size: [::core::ffi::c_char; 8],
    pub __align: ::core::ffi::c_long,
}
pub type pthread_spinlock_t = ::core::ffi::c_int;

#[repr(C)]
pub struct ss {
    pub m1: Mutex<ssm1Data>,

    pub m2: RwLock<ssm2Data>,

    pub m3: Mutex<ssm3Data>,
}
pub struct ssm1Data {
    pub n1: ::core::ffi::c_int,
}
pub struct ssm2Data {
    pub n2: ::core::ffi::c_int,
}
pub struct ssm3Data {
    pub n3: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn f1(mut s: *mut ss) {
    let mut s_m1_guard; // Will be assigned by lock/trylock
    let mut s_m2_guard; // Will be assigned by lock/trylock
    let mut s_m3_guard; // Will be assigned by lock/trylock

    s_m1_guard = (*s).m1.lock().unwrap();
    (*s_m1_guard).n1 += 1 as ::core::ffi::c_int;
    drop(s_m1_guard);
    s_m2_guard = (*s).m2.write().unwrap();
    (*s_m2_guard).n2 += 1 as ::core::ffi::c_int;
    drop(s_m2_guard);
    s_m3_guard = (*s).m3.lock().unwrap();
    (*s_m3_guard).n3 += 1 as ::core::ffi::c_int;
    drop(s_m3_guard);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1(arg as *mut ss);
    return NULL;
}
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut s: ss = ss {
        m1: Mutex::new(ssm1Data { n1: 0 }),

        m2: RwLock::new(ssm2Data { n2: 0 }),

        m3: Mutex::new(ssm3Data { n3: 0 }),
    };
    ();
    s.m1 = Mutex::new(ssm1Data {
        n1: 1 as ::core::ffi::c_int,
    });
    ();
    s.m2 = RwLock::new(ssm2Data {
        n2: 2 as ::core::ffi::c_int,
    });
    ();
    s.m3 = Mutex::new(ssm3Data {
        n3: 3 as ::core::ffi::c_int,
    });
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &raw mut id1,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        &raw mut s as *mut ::core::ffi::c_void,
    );
    pthread_create(
        &raw mut id2,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        &raw mut s as *mut ::core::ffi::c_void,
    );
    pthread_join(id1, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_join(id2, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    printf(
        b"%d %d %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        s.m1.get_mut().unwrap().n1,
        s.m2.get_mut().unwrap().n2,
        s.m3.get_mut().unwrap().n3,
    );
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
