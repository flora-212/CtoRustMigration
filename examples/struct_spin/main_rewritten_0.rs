use ::libc;
use std::ptr;
use std::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct ss {
    pub n1: libc::c_int,
    pub m1: Mutex<libc::c_int>,
    pub n2: libc::c_int,
    pub m2: RwLock<libc::c_int>,
    pub n3: libc::c_int,
    pub m3: Mutex<libc::c_int>, // Replaced Spinlock with Mutex
}

extern "C" {
    fn pthread_create(
        __newthread: *mut libc::c_ulong,
        __attr: *const libc::c_void,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}

unsafe extern "C" fn f1(s: *mut ss) {
    let s = &mut *s;
    *s.m1.lock().unwrap() += 1;
    *s.m2.write().unwrap() += 1;
    *s.m3.lock().unwrap() += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(arg as *mut ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut s = ss {
        n1: 0,
        m1: Mutex::new(0),
        n2: 0,
        m2: RwLock::new(0),
        n3: 0,
        m3: Mutex::new(0), // Replaced Spinlock with Mutex
    };

    s.n1 = 1;
    s.n2 = 2;
    s.n3 = 3;

    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    pthread_create(
        &mut id1,
        ptr::null(),
        Some(t_fun),
        &mut s as *mut ss as *mut libc::c_void,
    );
    pthread_create(
        &mut id2,
        ptr::null(),
        Some(t_fun),
        &mut s as *mut ss as *mut libc::c_void,
    );

    pthread_join(id1, ptr::null_mut());
    pthread_join(id2, ptr::null_mut());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
