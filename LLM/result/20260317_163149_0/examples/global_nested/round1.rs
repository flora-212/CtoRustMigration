use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static mut N2: i32 = 0;

fn f1() {
    unsafe {
        N1 += 1;
        N2 += 1;
        N1 += 1;
        N1 += N2;
        N1 += 1;
        N2 += 1;
        N2 += N1;
        N2 += 1;
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