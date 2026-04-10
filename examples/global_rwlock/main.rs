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
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
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
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_RWLOCK_DEFAULT_NP: C2Rust_Unnamed = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: C2Rust_Unnamed = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
#[no_mangle]
pub static mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut lock: pthread_rwlock_t = pthread_rwlock_t {
    __data: __pthread_rwlock_arch_t {
        __readers: 0 as ::core::ffi::c_uint,
        __writers: 0 as ::core::ffi::c_uint,
        __wrphase_futex: 0 as ::core::ffi::c_uint,
        __writers_futex: 0 as ::core::ffi::c_uint,
        __pad3: 0 as ::core::ffi::c_uint,
        __pad4: 0 as ::core::ffi::c_uint,
        __cur_writer: 0 as ::core::ffi::c_int,
        __shared: 0 as ::core::ffi::c_int,
        __rwelision: 0 as ::core::ffi::c_schar,
        __pad1: [
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        __pad2: 0 as ::core::ffi::c_ulong,
        __flags: PTHREAD_RWLOCK_DEFAULT_NP as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
};
#[no_mangle]
pub unsafe extern "C" fn f1() -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    pthread_rwlock_rdlock(&raw mut lock);
    x = n;
    pthread_rwlock_unlock(&raw mut lock);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn f2() {
    pthread_rwlock_wrlock(&raw mut lock);
    n += 1;
    pthread_rwlock_unlock(&raw mut lock);
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
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
        Some(
            t_fun
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        NULL,
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
        NULL,
    );
    pthread_join(id1, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_join(id2, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    printf(b"%d\n\0".as_ptr() as *const ::core::ffi::c_char, n);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
