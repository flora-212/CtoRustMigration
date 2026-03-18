use ::libc;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

pub struct SharedState {
    n1: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static S: SharedState = SharedState {
    n1: 0,
    m1: Mutex::new(()),
    cond: Condvar::new(),
};

unsafe extern "C" fn f1() {
    let mut s = S.m1.lock().unwrap();
    S.n1 += 1;
    if S.n1 == 1 {
        s = S.cond.wait(s).unwrap();
    } else {
        S.cond.notify_one();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );

    libc::pthread_join(id1, std::ptr::null_mut());
    libc::pthread_join(id2, std::ptr::null_mut());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
