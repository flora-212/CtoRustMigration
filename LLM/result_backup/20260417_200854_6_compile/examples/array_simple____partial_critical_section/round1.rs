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
        let x = *num_mutex[i].lock().unwrap();
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
            unsafe { t_fun(std::ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let print = |arr: &[i32]| {
        let c_str = CString::new(format!(
            "{} {} {} {} {}\n",
            arr[0], arr[1], arr[2], arr[3], arr[4]
        ))
        .unwrap();
        unsafe {
            libc::printf(c_str.as_ptr());
        }
    };

    print(&n1);
    print(&n2);
    print(&n3);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}