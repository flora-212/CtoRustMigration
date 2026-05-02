use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
static mut N1: i32 = 0;
#[no_mangle]
static mut N2: i32 = 0;

#[no_mangle]
static mut N1_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
#[no_mangle]
static mut N2_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
unsafe extern "C" fn f1() {
    let n1_mutex = &*N1_MUTEX;
    let n2_mutex = &*N2_MUTEX;

    *n1_mutex.lock().unwrap() += 1;
    *n2_mutex.lock().unwrap() += 1;
    *n1_mutex.lock().unwrap() += 1;
    *n1_mutex.lock().unwrap() += *n2_mutex.lock().unwrap();
    *n1_mutex.lock().unwrap() += 1;
    *n2_mutex.lock().unwrap() += 1;
    *n2_mutex.lock().unwrap() += *n1_mutex.lock().unwrap();
    *n2_mutex.lock().unwrap() += 1;
}

#[no_mangle]
unsafe extern "C" fn f2() {
    let n1_mutex = &*N1_MUTEX;
    let n2_mutex = &*N2_MUTEX;

    *n2_mutex.lock().unwrap() += 1;
    *n1_mutex.lock().unwrap() += 1;
    *n2_mutex.lock().unwrap() += 1;
    *n2_mutex.lock().unwrap() += *n1_mutex.lock().unwrap();
    *n2_mutex.lock().unwrap() += 1;
    *n1_mutex.lock().unwrap() += 1;
    *n1_mutex.lock().unwrap() += *n2_mutex.lock().unwrap();
    *n1_mutex.lock().unwrap() += 1;
}

#[no_mangle]
unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    if arg as libc::c_long == 0 {
        f1();
    } else {
        f2();
    }
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let n1_mutex = Arc::clone(&N1_MUTEX);
    let n2_mutex = Arc::clone(&N2_MUTEX);

    let handle1 = thread::spawn(move || {
        f1();
    });

    let handle2 = thread::spawn(move || {
        f2();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d %d\n\0".as_ptr() as *const libc::c_char, N1, N2);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}