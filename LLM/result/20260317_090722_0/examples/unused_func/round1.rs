use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static NUM_MUTEX: Mutex<i32> = Mutex::new(0);

fn f1() {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    *num_mutex += 1;
}

fn f2() {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    *num_mutex += 1;
}

fn t_fun() {
    f1();
}

fn main() {
    let handle1 = thread::spawn(|| {
        t_fun();
    });

    let handle2 = thread::spawn(|| {
        t_fun();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
