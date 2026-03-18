use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static NUM_MUTEX: Mutex<i32> = Mutex::new(0);

fn lock() {
    let _guard = NUM_MUTEX.lock().unwrap();
}

fn unlock() {
    // No-op, as the guard will automatically unlock when dropped
}

fn f1() {
    let mut n1 = NUM_MUTEX.lock().unwrap();
    *n1 += 1;
}

fn lock2(n: i32) -> i32 {
    let mut n1 = NUM_MUTEX.lock().unwrap();
    *n1 += n;
    *n1
}

fn unlock2(n: i32) -> i32 {
    let mut n1 = NUM_MUTEX.lock().unwrap();
    *n1 += n;
    *n1
}

fn f2() -> i32 {
    let n2 = lock2(1);
    f1();
    let n2 = unlock2(1);
    n2
}

fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    let handle1 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });
    let handle2 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}