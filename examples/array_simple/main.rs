use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 5;

#[no_mangle]
static mut n1: [i32; N] = [0; N];

#[no_mangle]
static mut n2: [i32; N] = [1, 2, 3, 4, 5];

#[no_mangle]
static mut n3: [i32; N] = [1; N];

#[no_mangle]
static mut num_mutex: [Mutex<i32>; N] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];

#[no_mangle]
unsafe extern "C" fn f1() {
    for i in 0..N {
        let x = n3[i];
        let mut num_mutex_i = num_mutex[i].lock().unwrap();
        n1[i] += x;
        n2[i] += x;
    }
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            f1();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
