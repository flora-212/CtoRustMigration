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

fn t_fun() {
    f1();
    f2();
}

fn main() {
    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();
}
