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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss {
    pub n: ::core::ffi::c_int,
    pub m: pthread_mutex_t,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub static mut lock: pthread_mutex_t = pthread_mutex_t {
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
pub static mut x: *mut ss = ::core::ptr::null::<ss>() as *mut ss;
#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut s: *mut ss = malloc(::core::mem::size_of::<ss>() as size_t) as *mut ss;
    (*s).n = 123 as ::core::ffi::c_int;
    pthread_mutex_init(&raw mut (*s).m, ::core::ptr::null::<pthread_mutexattr_t>());
    x = s;
    pthread_mutex_lock(&raw mut (*s).m);
    (*s).n = 456 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut (*s).m);
}
#[no_mangle]
pub unsafe extern "C" fn f2() {
    let mut s: *mut ss = malloc(::core::mem::size_of::<ss>() as size_t) as *mut ss;
    pthread_mutex_init(&raw mut (*s).m, ::core::ptr::null::<pthread_mutexattr_t>());
    x = s;
    pthread_mutex_lock(&raw mut lock);
    (*s).n = 789 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut lock);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1();
    f2();
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
