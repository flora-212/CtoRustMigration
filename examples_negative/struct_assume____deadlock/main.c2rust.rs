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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss {
    pub n: ::core::ffi::c_int,
    pub m: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args {
    pub s1: *mut ss,
    pub s2: *mut ss,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn inc(mut s: *mut ss) {
    (*s).n = (*s).n + 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn f1(mut s1: *mut ss, mut s2: *mut ss) {
    pthread_mutex_lock(&raw mut (*s1).m);
    pthread_mutex_lock(&raw mut (*s2).m);
    inc(s1);
    inc(s2);
    pthread_mutex_unlock(&raw mut (*s2).m);
    pthread_mutex_unlock(&raw mut (*s1).m);
}
#[no_mangle]
pub unsafe extern "C" fn f2(mut s1: *mut ss, mut s2: *mut ss) {
    pthread_mutex_lock(&raw mut (*s2).m);
    pthread_mutex_lock(&raw mut (*s1).m);
    inc(s1);
    inc(s2);
    pthread_mutex_unlock(&raw mut (*s1).m);
    pthread_mutex_unlock(&raw mut (*s2).m);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun1(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut a: *mut args = arg as *mut args;
    f1((*a).s1, (*a).s2);
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn t_fun2(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut a: *mut args = arg as *mut args;
    f2((*a).s1, (*a).s2);
    return NULL;
}
unsafe fn main_0() -> ::core::ffi::c_int {
    let mut s1: *mut ss = ::core::ptr::null_mut::<ss>();
    let mut s2: *mut ss = ::core::ptr::null_mut::<ss>();
    s1 = malloc(::core::mem::size_of::<ss>() as size_t) as *mut ss;
    s2 = malloc(::core::mem::size_of::<ss>() as size_t) as *mut ss;
    (*s1).n = 0 as ::core::ffi::c_int;
    let mut a: args = args { s1, s2 };
    pthread_mutex_init(&raw mut (*s1).m, ::core::ptr::null::<pthread_mutexattr_t>());
    pthread_mutex_init(&raw mut (*s2).m, ::core::ptr::null::<pthread_mutexattr_t>());
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &raw mut id1,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun1 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        &raw mut a as *mut ::core::ffi::c_void,
    );
    pthread_create(
        &raw mut id2,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(t_fun2 as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void),
        &raw mut a as *mut ::core::ffi::c_void,
    );
    pthread_join(id1, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_join(id2, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
