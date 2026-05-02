use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
pub static mut n1: i32 = 0;

#[no_mangle]
pub static mut n2: i32 = 0;

#[no_mangle]
pub static mut n3: i32 = 1;

#[no_mangle]
pub static mut num_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let x = n3;
    let mut num_mutex = num_mutex.lock().unwrap();
    *num_mutex += x;
    n1 += x;
    n2 += x;
    n3 += x;
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

    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        n1,
        n2,
        n3,
    );

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}