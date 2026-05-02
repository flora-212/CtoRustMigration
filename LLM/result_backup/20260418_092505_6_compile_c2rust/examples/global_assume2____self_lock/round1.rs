use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
pub static mut n1: i32 = 0;

#[no_mangle]
pub static mut num_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub unsafe extern "C" fn inc() -> i32 {
    n1 += 1;
    if n1 != 0 {
        n1
    } else {
        n1 + 1
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut num_mutex_clone = num_mutex.clone();
    let mut num_mutex_clone2 = num_mutex.clone();
    let _guard1 = num_mutex_clone.lock().unwrap();
    let _guard2 = num_mutex_clone2.lock().unwrap();
    inc();
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let num_mutex_clone = num_mutex.clone();
    let num_mutex_clone2 = num_mutex.clone();

    let handle1 = thread::spawn(move || {
        let _guard = num_mutex_clone.lock().unwrap();
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        let _guard = num_mutex_clone2.lock().unwrap();
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", n1);
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}