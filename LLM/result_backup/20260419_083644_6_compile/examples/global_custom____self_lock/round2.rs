use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[no_mangle]
static mut n1: i32 = 0;

#[no_mangle]
static mut num_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub extern "C" fn lock() {
    let _guard = num_mutex.lock().unwrap();
}

#[no_mangle]
pub extern "C" fn unlock() {
    // No-op, as the lock is released when the guard goes out of scope
}

#[no_mangle]
pub extern "C" fn f1() {
    let mut guard = num_mutex.lock().unwrap();
    *guard += 1;
}

#[no_mangle]
pub extern "C" fn lock2(n: i32) -> i32 {
    let mut guard = num_mutex.lock().unwrap();
    *guard += n;
    *guard
}

#[no_mangle]
pub extern "C" fn unlock2(n: i32) -> i32 {
    let mut guard = num_mutex.lock().unwrap();
    *guard += n;
    *guard
}

#[no_mangle]
pub extern "C" fn f2() -> i32 {
    let n2 = lock2(1);
    let mut guard = num_mutex.lock().unwrap();
    *guard += 1;
    *guard + n2
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    lock();
    f1();
    f2();
    unlock();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let num_mutex_clone = Arc::clone(&num_mutex);
    let handle1 = thread::spawn(move || {
        let _guard = num_mutex_clone.lock().unwrap();
        t_fun(ptr::null_mut());
    });

    let num_mutex_clone = Arc::clone(&num_mutex);
    let handle2 = thread::spawn(move || {
        let _guard = num_mutex_clone.lock().unwrap();
        t_fun(ptr::null_mut());
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