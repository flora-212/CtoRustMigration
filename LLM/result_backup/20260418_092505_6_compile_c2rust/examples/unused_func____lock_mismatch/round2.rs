use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
static mut n1: i32 = 0;

#[no_mangle]
static NUM_MUTEX1: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
static NUM_MUTEX2: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
unsafe extern "C" fn f1() {
    let mut num_mutex1 = NUM_MUTEX1.lock().unwrap();
    *num_mutex1 += 1;
}

#[no_mangle]
unsafe extern "C" fn f2() {
    let mut num_mutex2 = NUM_MUTEX2.lock().unwrap();
    *num_mutex2 += 1;
}

#[no_mangle]
unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let num_mutex1 = Arc::clone(&NUM_MUTEX1);
    let num_mutex2 = Arc::clone(&NUM_MUTEX2);

    let handle1 = thread::spawn(move || {
        f1();
    });

    let handle2 = thread::spawn(move || {
        f2();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}