use std::sync::{Arc, Mutex};
use std::thread;

pub struct SharedData {
    n1: i32,
    m1: Mutex<()>,
}

static mut S: SharedData = SharedData {
    n1: 0,
    m1: Mutex::new(()),
};

unsafe extern "C" fn f1() {
    let _guard = S.m1.lock().unwrap();
    S.n1 += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    S.n1 += 1;

    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    S.n1 += 1;

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
