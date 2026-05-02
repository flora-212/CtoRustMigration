use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
pub static mut N1: i32 = 0;

#[no_mangle]
pub static mut NUM_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    *num_mutex += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    N1 += 1;

    let num_mutex_clone = NUM_MUTEX.clone();
    let handle1 = thread::spawn(move || {
        let _ = num_mutex_clone.lock().unwrap();
        f1();
    });

    let num_mutex_clone = NUM_MUTEX.clone();
    let handle2 = thread::spawn(move || {
        let _ = num_mutex_clone.lock().unwrap();
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    N1 += 1;

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}