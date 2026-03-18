use std::sync::{Arc, Condvar, Mutex};
use std::thread;

static mut N1: i32 = 0;
static mut N2: i32 = 0;
static NUM_MUTEX: Mutex<()> = Mutex::new(());
static COND: Condvar = Condvar::new();

fn f1() {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    unsafe {
        N1 += 1;
        if N1 == 1 {
            num_mutex = COND.wait(num_mutex).unwrap();
        } else {
            COND.notify_one();
        }
    }

    num_mutex = NUM_MUTEX.lock().unwrap();
    unsafe {
        N2 += 1;
        if N2 == 1 {
            num_mutex = COND.wait(num_mutex).unwrap();
        } else {
            COND.notify_all();
        }
    }
}

fn t_fun() {
    f1();
}

fn main() {
    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();
}
