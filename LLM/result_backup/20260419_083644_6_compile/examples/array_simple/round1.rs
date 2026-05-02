use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_void;

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
unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
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

    let n1_str = CString::new(format!(
        "{} {} {} {} {}\n",
        n1[0], n1[1], n1[2], n1[3], n1[4]
    ))
    .unwrap();
    let n2_str = CString::new(format!(
        "{} {} {} {} {}\n",
        n2[0], n2[1], n2[2], n2[3], n2[4]
    ))
    .unwrap();
    let n3_str = CString::new(format!(
        "{} {} {} {} {}\n",
        n3[0], n3[1], n3[2], n3[3], n3[4]
    ))
    .unwrap();

    libc::printf(n1_str.as_ptr());
    libc::printf(n2_str.as_ptr());
    libc::printf(n3_str.as_ptr());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}