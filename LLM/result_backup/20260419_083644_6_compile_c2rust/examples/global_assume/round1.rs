use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
static mut N1: i32 = 0;

#[no_mangle]
static NUM_MUTEX: Mutex<i32> = Mutex::new(0);

#[no_mangle]
unsafe extern "C" fn inc() {
    let mut n1 = NUM_MUTEX.lock().unwrap();
    *n1 += 1;
}

#[no_mangle]
unsafe extern "C" fn f1() {
    inc();
}

#[no_mangle]
unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}