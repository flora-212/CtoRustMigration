use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
    });

    let shared_data_clone = shared_data.clone();
    let handle1 = thread::spawn(move || {
        let mut n1 = shared_data_clone.n1.lock().unwrap();
        *n1 += 1;
    });

    let shared_data_clone = shared_data.clone();
    let handle2 = thread::spawn(move || {
        let mut n1 = shared_data_clone.n1.lock().unwrap();
        *n1 += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let mut n1 = shared_data.n1.lock().unwrap();
    *n1 += 1;

    let c_string = CString::new(format!("{}\n", *n1)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

pub fn main() {
    std::process::exit(main_0() as i32);
}