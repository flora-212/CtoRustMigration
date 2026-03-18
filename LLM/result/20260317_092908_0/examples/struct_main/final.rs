use std::sync::{Arc, Mutex};
use std::thread;

pub struct SharedData {
    n1: i32,
    m1: Mutex<i32>,
}

static mut S: SharedData = SharedData {
    n1: 0,
    m1: Mutex::new(0),
};

unsafe extern "C" fn f1() {
    let mut m1 = S.m1.lock().unwrap();
    S.n1 += 1;
}

fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    unsafe { f1() };
    std::ptr::null_mut()
}

fn main() {
    unsafe {
        S.n1 += 1;
    }

    let handle1 = thread::spawn(|| t_fun(std::ptr::null_mut()));
    let handle2 = thread::spawn(|| t_fun(std::ptr::null_mut()));

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        S.n1 += 1;
    }
}
