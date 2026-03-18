use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static NUM_MUTEX: Mutex<i32> = Mutex::new(0);

fn lock() {
    NUM_MUTEX.lock().unwrap();
}

fn unlock() {
    drop(NUM_MUTEX.lock().unwrap());
}

fn f1() {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    *num_mutex += 1;
}

fn lock2(n: i32) -> i32 {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    *num_mutex += n;
    *num_mutex
}

fn unlock2(n: i32) -> i32 {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    *num_mutex += n;
    *num_mutex
}

fn f2() -> i32 {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    *num_mutex += 1;
    let n2 = *num_mutex;
    *num_mutex += 1;
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
