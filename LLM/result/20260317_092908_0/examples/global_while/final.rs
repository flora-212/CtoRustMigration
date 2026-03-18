use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static NUM_MUTEX: Mutex<i32> = Mutex::new(0);

fn f1() {
    let mut x: i32;
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    while unsafe { N1 } < 10 {
        x = unsafe { N1 };
        drop(num_mutex);
        x += 1;
        num_mutex = NUM_MUTEX.lock().unwrap();
        unsafe { N1 = x };
    }
}

fn f2() {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    while unsafe { N1 } < 20 {
        if unsafe { N1 } > 18 {
            drop(num_mutex);
            return;
        }
        unsafe { N1 += 1 };
    }
}

fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    0 as *mut libc::c_void
}

fn main_0() -> i32 {
    let handle1 = thread::spawn(|| unsafe { t_fun(0 as *mut libc::c_void) });
    let handle2 = thread::spawn(|| unsafe { t_fun(0 as *mut libc::c_void) });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}
