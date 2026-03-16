use ::libc;
use std::sync::Mutex;
use std::thread;

extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
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

static n1: [Mutex<libc::c_int>; 3] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];

static num_mutex: [Mutex<()>; 3] = [
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
];

fn f1() {
    let mut n1 = n1.iter().map(|m| m.lock().unwrap()).collect::<Vec<_>>();
    *n1[0] += 1;
    *n1[1] += 1;
    *n1[2] += 1;
}

unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;

    pthread_create(
        &mut id1 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );

    pthread_create(
        &mut id2 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );

    pthread_join(id1, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut libc::c_void as *mut *mut libc::c_void);

    0 as libc::c_int
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
