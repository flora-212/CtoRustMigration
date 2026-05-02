use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<i32>,
}

lazy_static::lazy_static! {
    #[no_mangle]
    pub static ref S1: Arc<St> = Arc::new(St {
        n1: 0,
        num_mutex: Mutex::new(0),
    });

    #[no_mangle]
    pub static ref S2: Arc<St> = Arc::new(St {
        n1: 1,
        num_mutex: Mutex::new(0),
    });

    #[no_mangle]
    pub static ref S3: Arc<St> = Arc::new(St {
        n1: 2,
        num_mutex: Mutex::new(0),
    });
}

#[no_mangle]
pub extern "C" fn h(s: &Arc<St>) {
    let mut n1 = s.num_mutex.lock().unwrap();
    *n1 += 1;
}

#[no_mangle]
pub extern "C" fn g(s: &Arc<St>) {
    let mut n1 = s.num_mutex.lock().unwrap();
    *n1 += 1;
    h(s);
}

#[no_mangle]
pub extern "C" fn f(s: &Arc<St>) {
    let mut n1 = s.num_mutex.lock().unwrap();
    *n1 += 1;
    g(s);
}

#[no_mangle]
pub extern "C" fn f1() {
    f(&S1);
    f(&S2);
    f(&S3);
}

#[no_mangle]
pub extern "C" fn t_fun(_: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);
    let s3 = Arc::clone(&S3);

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", S1.n1, S2.n1, S3.n1);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
