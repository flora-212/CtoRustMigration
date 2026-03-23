extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
}
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub n1: ::core::ffi::c_int,
    pub n2: ::core::ffi::c_int,
    pub n3: ::core::ffi::c_int,
    pub m1: pthread_mutex_t,
    pub cond: pthread_cond_t,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub static mut s: C2Rust_Unnamed_1 = C2Rust_Unnamed_1 {
    n1: 0 as ::core::ffi::c_int,
    n2: 0 as ::core::ffi::c_int,
    n3: 0 as ::core::ffi::c_int,
    m1: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0 as ::core::ffi::c_int,
            __count: 0 as ::core::ffi::c_uint,
            __owner: 0 as ::core::ffi::c_int,
            __nusers: 0 as ::core::ffi::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
            __spins: 0 as ::core::ffi::c_short,
            __elision: 0 as ::core::ffi::c_short,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
                __next: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
            },
        },
    },
    cond: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter {
                __value64: 0 as ::core::ffi::c_int as ::core::ffi::c_ulonglong,
            },
            __g1_start: __atomic_wide_counter {
                __value64: 0 as ::core::ffi::c_int as ::core::ffi::c_ulonglong,
            },
            __g_size: [
                0 as ::core::ffi::c_int as ::core::ffi::c_uint,
                0 as ::core::ffi::c_int as ::core::ffi::c_uint,
            ],
            __g1_orig_size: 0 as ::core::ffi::c_uint,
            __wrefs: 0 as ::core::ffi::c_uint,
            __g_signals: [
                0 as ::core::ffi::c_int as ::core::ffi::c_uint,
                0 as ::core::ffi::c_int as ::core::ffi::c_uint,
            ],
            __unused_initialized_1: 0 as ::core::ffi::c_uint,
            __unused_initialized_2: 0 as ::core::ffi::c_uint,
        },
    },
};
#[no_mangle]
pub static mut m2: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __elision: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
        },
    },
};
#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    pthread_mutex_lock(&raw mut s.m1);
    pthread_mutex_lock(&raw mut m2);
    s.n1 = s.n1 + 1 as ::core::ffi::c_int;
    if s.n1 == 1 as ::core::ffi::c_int {
        clock_gettime(0 as clockid_t, &raw mut ts);
        ts.tv_sec += 1 as ::core::ffi::c_int as time_t as __time_t;
        pthread_cond_timedwait(&raw mut s.cond, &raw mut s.m1, &raw mut ts);
    } else {
        pthread_cond_signal(&raw mut s.cond);
    }
    pthread_mutex_unlock(&raw mut m2);
    pthread_mutex_unlock(&raw mut s.m1);
}
#[no_mangle]
pub unsafe extern "C" fn f2() {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    pthread_mutex_lock(&raw mut m2);
    pthread_mutex_lock(&raw mut s.m1);
    s.n2 = s.n2 + 1 as ::core::ffi::c_int;
    if s.n2 == 1 as ::core::ffi::c_int {
        clock_gettime(0 as clockid_t, &raw mut ts);
        ts.tv_nsec += 1 as ::core::ffi::c_int as ::core::ffi::c_long;
        pthread_cond_timedwait(&raw mut s.cond, &raw mut s.m1, &raw mut ts);
    } else {
        pthread_cond_signal(&raw mut s.cond);
    }
    pthread_mutex_unlock(&raw mut s.m1);
    pthread_mutex_unlock(&raw mut m2);
}
#[no_mangle]
pub unsafe extern "C" fn f3() {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    pthread_mutex_lock(&raw mut s.m1);
    s.n3 = s.n3 + 1 as ::core::ffi::c_int;
    if s.n3 == 1 as ::core::ffi::c_int {
        clock_gettime(0 as clockid_t, &raw mut ts);
        ts.tv_sec += 1 as ::core::ffi::c_int as time_t as __time_t;
        ts.tv_nsec += 2 as ::core::ffi::c_int as ::core::ffi::c_long;
        pthread_cond_timedwait(&raw mut s.cond, &raw mut s.m1, &raw mut ts);
    } else {
        pthread_cond_signal(&raw mut s.cond);
    }
    pthread_mutex_unlock(&raw mut s.m1);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1();
    f2();
    f3();
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
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
