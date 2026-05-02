use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::ptr;

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
pub unsafe extern "C" fn f1() {
    for i in 0..N {
        let x = n3[i];
        n1[i] += x;
        n2[i] += x;
    }
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
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

    let n1_str = n1.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ");
    let n2_str = n2.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ");
    let n3_str = n3.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ");

    println!("{}", n1_str);
    println!("{}", n2_str);
    println!("{}", n3_str);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}