use ::libc;
extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}

pub type pthread_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}

pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss1 {
    pub n1: libc::c_int,
    pub n2: libc::c_int,
    pub m1: pthread_mutex_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss2 {
    pub n1: libc::c_int,
    pub n3: libc::c_int,
    pub m2: pthread_mutex_t,
}

static mut s1: ss1 = ss1 {
    n1: 0,
    n2: 1,
    m1: __anonunion_pthread_mutex_t_335460617 {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: core::ptr::null_mut(),
                __next: core::ptr::null_mut(),
            },
        },
    },
};

static mut s2: ss2 = ss2 {
    n1: 2,
    n3: 3,
    m2: __anonunion_pthread_mutex_t_335460617 {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: core::ptr::null_mut(),
                __next: core::ptr::null_mut(),
            },
        },
    },
};

unsafe extern "C" fn f1() {
    let x = s1.n2 + s2.n3;
    pthread_mutex_lock(&mut s1.m1);
    pthread_mutex_lock(&mut s2.m2);
    s1.n1 += x;
    s2.n1 += x;
    pthread_mutex_unlock(&mut s1.m1);
    pthread_mutex_unlock(&mut s2.m2);
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    core::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &mut id1,
        core::ptr::null(),
        t_fun,
        core::ptr::null_mut(),
    );
    pthread_create(
        &mut id2,
        core::ptr::null(),
        t_fun,
        core::ptr::null_mut(),
    );
    pthread_join(id1, core::ptr::null_mut());
    pthread_join(id2, core::ptr::null_mut());
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
