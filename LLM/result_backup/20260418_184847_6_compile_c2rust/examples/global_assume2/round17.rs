use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::c_void;
use std::sync::Once;

#[no_mangle]
static mut n1: i32 = 0;

#[no_mangle]
static NUM_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
unsafe extern "C" fn inc() -> i32 {
    let mut n1_lock = NUM_MUTEX.lock().unwrap();
    *n1_lock += 1;
    if *n1_lock != 0 {
        *n1_lock
    } else {
        *n1_lock + 1
    }
}

#[no_mangle]
unsafe extern "C" fn f1() {
    let _guard = NUM_MUTEX.lock().unwrap();
    inc();
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let num_mutex_clone = Arc::clone(&NUM_MUTEX);

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1_value = *NUM_MUTEX.lock().unwrap();
    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1_value);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}