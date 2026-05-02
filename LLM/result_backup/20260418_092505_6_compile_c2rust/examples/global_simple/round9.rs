use std::sync::{Arc, Mutex};
use std::thread;

#[no_mangle]
pub static mut n1: i32 = 0;

#[no_mangle]
pub static mut n2: i32 = 0;

#[no_mangle]
pub static mut n3: i32 = 1;

#[no_mangle]
pub static mut num_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub fn f1() {
    let x = n3;
    let mut num_mutex_lock = num_mutex.lock().unwrap();
    *num_mutex_lock += x;
    n1 += x;
    n2 += x;
}

#[no_mangle]
pub fn t_fun() {
    f1();
}

unsafe fn main_0() -> i32 {
    let num_mutex_clone = Arc::clone(&num_mutex);

    let handle1 = thread::spawn(move || {
        t_fun();
    });

    let handle2 = thread::spawn(move || {
        t_fun();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", n1, n2, n3);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}