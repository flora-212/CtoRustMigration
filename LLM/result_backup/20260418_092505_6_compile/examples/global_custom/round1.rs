use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
static mut n1: i32 = 0;

#[no_mangle]
static mut num_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub unsafe extern "C" fn lock() {
    num_mutex.lock().unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn unlock() {
    drop(num_mutex.lock().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    lock();
    n1 += 1;
    unlock();
}

#[no_mangle]
pub unsafe extern "C" fn lock2(mut n: i32) -> i32 {
    let mut guard = num_mutex.lock().unwrap();
    n1 += n;
    *guard = n1;
    n1
}

#[no_mangle]
pub unsafe extern "C" fn unlock2(mut n: i32) -> i32 {
    let mut guard = num_mutex.lock().unwrap();
    n1 += n;
    *guard = n1;
    n1
}

#[no_mangle]
pub unsafe extern "C" fn f2() -> i32 {
    let n2 = lock2(1);
    n1 += 1;
    let n2 = unlock2(1);
    n2
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
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