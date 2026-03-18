use ::libc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
#[repr(C)]
pub struct ss0 {
    pub n1: libc::c_int,
    pub n2: libc::c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct ss {
    pub s: ss0,
    pub m: Arc<Mutex<()>>,
}

pub static mut s: ss = {
    let mut init = ss {
        s: ss0 {
            n1: 0 as libc::c_int,
            n2: 1 as libc::c_int,
        },
        m: Arc::new(Mutex::new(())),
    };
    init
};

pub fn f1() {
    let _guard = s.m.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

extern "C" {
    fn pthread_create(
        __newthread: *mut libc::pthread_t,
        __attr: *const libc::pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: libc::pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}

pub fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    pthread_create(
        &mut id1 as *mut libc::pthread_t,
        0 as *mut libc::c_void as *const libc::pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );

    pthread_create(
        &mut id2 as *mut libc::pthread_t,
        0 as *mut libc::c_void as *const libc::pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );

    pthread_join(id1, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut libc::c_void as *mut *mut libc::c_void);

    0 as libc::c_int
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
