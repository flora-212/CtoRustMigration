use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;

const N: c_int = 5;

#[no_mangle]
static mut n1: [c_int; 5] = [0; 5];

#[no_mangle]
static mut num_mutex: [Arc<Mutex<c_int>>; 5] = [
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
];

#[no_mangle]
unsafe extern "C" fn f1() {
    let mut i: c_int = 0;
    while i < N {
        let mut num = num_mutex[i as usize].lock().unwrap();
        *num += 1;
        *num *= 2;
        i += 1;
    }
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    let mut i: c_int = 0;
    while i < N {
        n1[i as usize] += 1;
        i += 1;
    }

    let mut handles = vec![];

    let handle1 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });
    handles.push(handle1);

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    i = 0;
    while i < N {
        n1[i as usize] += 1;
        i += 1;
    }

    let format = CString::new("%d %d %d %d %d\n").unwrap();
    unsafe {
        libc::printf(
            format.as_ptr(),
            n1[0] as c_int,
            n1[1] as c_int,
            n1[2] as c_int,
            n1[3] as c_int,
            n1[4] as c_int,
        );
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}