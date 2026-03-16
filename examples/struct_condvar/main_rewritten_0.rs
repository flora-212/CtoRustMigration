use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[derive(Debug)]
struct SharedState {
    n1: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static mut S: SharedState = SharedState {
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
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    let t1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let t2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    t1.join().unwrap();
    t2.join().unwrap();

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
