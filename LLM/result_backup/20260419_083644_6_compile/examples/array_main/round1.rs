use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

const N: usize = 5;

#[no_mangle]
static mut n1: [i32; N] = [0; N];

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
        let mut num = num_mutex[i].lock().unwrap();
        *num += 1;
    }
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    for i in 0..N {
        n1[i] += 1;
    }

    let mut id1 = 0;
    let mut id2 = 0;

    let handle1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    for i in 0..N {
        n1[i] += 1;
    }

    libc::printf(
        b"%d %d %d %d %d\n\0".as_ptr() as *const libc::c_char,
        n1[0],
        n1[1],
        n1[2],
        n1[3],
        n1[4],
    );

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}