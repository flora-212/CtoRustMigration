use std::sync::{Arc, RwLock};
use std::thread;

static mut N: i32 = 0;
static LOCK: RwLock<i32> = RwLock::new(0);

fn f1() -> i32 {
    let lock = LOCK.read().unwrap();
    *lock
}

fn f2() {
    let mut lock = LOCK.write().unwrap();
    *lock += 1;
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
