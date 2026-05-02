use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::CString;

static INIT: Once = Once::new();

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

#[no_mangle]
pub static S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub extern "C" fn h(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

#[no_mangle]
pub extern "C" fn g(s: &Arc<St>) {
    s.n1 += 1;
    h(s);
}

#[no_mangle]
pub extern "C" fn f(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
}

#[no_mangle]
pub extern "C" fn f1() {
    f(&S1);
    f(&S2);
    f(&S3);
}

#[no_mangle]
pub extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(|| {
        // Initialize any global state here if needed
    });

    let s1 = S1.clone();
    let s2 = S2.clone();
    let s3 = S3.clone();

    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1_n1 = S1.n1;
    let s2_n1 = S2.n1;
    let s3_n1 = S3.n1;

    let message = format!("{} {} {}\n", s1_n1, s2_n1, s3_n1);
    let c_message = CString::new(message).unwrap();
    unsafe {
        libc::printf(c_message.as_ptr());
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}