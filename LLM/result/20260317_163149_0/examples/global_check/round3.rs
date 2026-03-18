use std::sync::{Arc, Mutex};
use std::thread;

static mut N: i32 = 0;
static M: Mutex<i32> = Mutex::new(0);

fn f1() {
    let mut n = M.lock().unwrap();
    *n += 1;
    drop(n);
    let mut n = M.lock().unwrap();
    *n += 1;
}

fn f2() {
    let mut n = M.lock().unwrap();
    *n += 1;
    drop(n);
    if let Ok(mut n) = M.try_lock() {
        *n += 1;
    }
}

fn f3() {
    let mut n = M.lock().unwrap();
    *n += 1;
    drop(n);
    let mut n = M.lock().unwrap();
    *n += 1;
    drop(n);
    if let Ok(mut n) = M.try_lock() {
        *n += 1;
    }
}

fn f4() {
    let mut n = M.lock().unwrap();
    *n += 1;
    drop(n);
    if let Ok(mut n) = M.try_lock() {
        *n += 1;
    }
    drop(n);
    let mut n = M.lock().unwrap();
    *n += 1;
}

fn t_fun() {
    f1();
    f2();
    f3();
    f4();
}

fn main() {
    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();
}