use std::sync::{Arc, RwLock};
use std::thread;

static N: RwLock<i32> = RwLock::new(0);

fn f1() -> i32 {
    let n = N.read().unwrap();
    *n
}

fn f2() {
    let mut n = N.write().unwrap();
    *n += 1;
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