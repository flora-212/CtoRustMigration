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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss {
    pub n1: ::core::ffi::c_int,
    pub m1: pthread_mutex_t,
    pub n2: ::core::ffi::c_int,
    pub m2: pthread_rwlock_t,
    pub n3: ::core::ffi::c_int,
    pub m3: pthread_spinlock_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn f1(mut s: *mut ss) {
    pthread_mutex_lock(&raw mut (*s).m1);
    (*s).n1 += 1 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut (*s).m1);
    pthread_rwlock_wrlock(&raw mut (*s).m2);
    (*s).n2 += 1 as ::core::ffi::c_int;
    pthread_rwlock_unlock(&raw mut (*s).m2);
    pthread_spin_lock(&raw mut (*s).m3);
    (*s).n3 += 1 as ::core::ffi::c_int;
    pthread_spin_unlock(&raw mut (*s).m3);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    f1(arg as *mut ss);
    return NULL;
}
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut s: ss = ss {
        n1: 0,
        m1: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                },
            },
        },
        n2: 0,
        m2: pthread_rwlock_t {
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
                __flags: 0,
            },
        },
        n3: 0,
        m3: 0,
    };
    s.n1 = 1 as ::core::ffi::c_int;
    pthread_mutex_init(&raw mut s.m1, ::core::ptr::null::<pthread_mutexattr_t>());
    s.n2 = 2 as ::core::ffi::c_int;
    pthread_rwlock_init(&raw mut s.m2, ::core::ptr::null::<pthread_rwlockattr_t>());
    s.n3 = 3 as ::core::ffi::c_int;
    pthread_spin_init(&raw mut s.m3, NULL as ::core::ffi::c_int);
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &raw mut id1,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            t_fun
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        &raw mut s as *mut ::core::ffi::c_void,
    );
    pthread_create(
        &raw mut id2,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            t_fun
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        &raw mut s as *mut ::core::ffi::c_void,
    );
    pthread_join(id1, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_join(id2, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
