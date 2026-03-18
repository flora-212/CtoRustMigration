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
pub struct __anonstruct_st_672045599 {
    pub n1: libc::c_int,
    pub num_mutex: pthread_mutex_t,
}

pub type st = __anonstruct_st_672045599;

static mut s1: st = st {
    n1: 0,
    num_mutex: __anonunion_pthread_mutex_t_335460617 {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: 0 as *mut __pthread_internal_list,
                __next: 0 as *mut __pthread_internal_list,
            },
        },
    },
};

static mut s2: st = st {
    n1: 1,
    num_mutex: __anonunion_pthread_mutex_t_335460617 {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: 0 as *mut __pthread_internal_list,
                __next: 0 as *mut __pthread_internal_list,
            },
        },
    },
};

static mut s3: st = st {
    n1: 2,
    num_mutex: __anonunion_pthread_mutex_t_335460617 {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: 0 as *mut __pthread_internal_list,
                __next: 0 as *mut __pthread_internal_list,
            },
        },
    },
};

unsafe extern "C" fn f(s: *mut st) {
    pthread_mutex_lock(&mut (*s).num_mutex);
    (*s).n1 += 1;
    g(s);
    pthread_mutex_unlock(&mut (*s).num_mutex);
}

unsafe extern "C" fn g(t: *mut st) {
    (*t).n1 += 1;
    h(t);
}

unsafe extern "C" fn h(u: *mut st) {
    (*u).n1 += 1;
}

unsafe extern "C" fn f1() {
    f(&mut s1);
    f(&mut s2);
    f(&mut s3);
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &mut id1,
        0 as *const pthread_attr_t,
        t_fun,
        0 as *mut libc::c_void,
    );
    pthread_create(
        &mut id2,
        0 as *const pthread_attr_t,
        t_fun,
        0 as *mut libc::c_void,
    );
    pthread_join(id1, 0 as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut *mut libc::c_void);
    0 as libc::c_int
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
