use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

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
        n1[i] += x;
        n2[i] += x;
    }
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        libc::printf(
            b"%d %d %d %d %d\n\0".as_ptr(),
            n1[0],
            n1[1],
            n1[2],
            n1[3],
            n1[4],
        );
        libc::printf(
            b"%d %d %d %d %d\n\0".as_ptr(),
            n2[0],
            n2[1],
            n2[2],
            n2[3],
            n2[4],
        );
        libc::printf(
            b"%d %d %d %d %d\n\0".as_ptr(),
            n3[0],
            n3[1],
            n3[2],
            n3[3],
            n3[4],
        );
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}