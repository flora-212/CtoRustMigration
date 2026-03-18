use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: [i32; 3] = [0; 3];
static NUM_MUTEX: [Mutex<i32>; 3] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];

fn f1() {
    let mut num_mutex = NUM_MUTEX.iter().map(|m| m.lock().unwrap());
    let mut first = num_mutex.next().unwrap();
    *first += 1;
    drop(first);

    let mut second = num_mutex.next().unwrap();
    *second += 1;
    drop(second);

    let mut third = num_mutex.next().unwrap();
    *third += 1;
    drop(third);
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
