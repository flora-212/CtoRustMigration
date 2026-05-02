use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
pub static mut n1: i32 = 0;

#[no_mangle]
pub static mut num_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub unsafe extern "C" fn inc() {
    let mut n1_lock = num_mutex.lock().unwrap();
    *n1_lock += 1;
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    inc();
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let num_mutex_clone = Arc::clone(&num_mutex);

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1);
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}