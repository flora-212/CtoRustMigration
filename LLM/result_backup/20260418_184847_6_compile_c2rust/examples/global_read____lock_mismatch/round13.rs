use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
static mut n1: i32 = 0;
#[no_mangle]
static mut n2: i32 = 0;

#[no_mangle]
static NUM_MUTEX1: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
#[no_mangle]
static NUM_MUTEX2: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
unsafe extern "C" fn f1() {
    let mut num_mutex1 = NUM_MUTEX1.lock().unwrap();
    n1 = n1 + n2;
}

#[no_mangle]
unsafe extern "C" fn f2() {
    let mut num_mutex2 = NUM_MUTEX2.lock().unwrap();
    n1 = n1 + n2;
}

#[no_mangle]
unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    if arg as libc::c_long == 0 {
        f1();
    } else {
        f2();
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut handles = vec![];

    let handle1 = thread::spawn(|| {
        let arg = std::ptr::null_mut();
        t_fun(arg);
    });
    handles.push(handle1);

    let handle2 = thread::spawn(|| {
        let arg = 1 as *mut libc::c_void;
        t_fun(arg);
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    let c_str = CString::new(format!("{} {}\n", n1, n2)).unwrap();
    libc::printf(c_str.as_ptr() as *const c_char);
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}