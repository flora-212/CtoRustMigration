use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;

#[repr(C)]
pub struct st {
    pub n1: i32,
    pub num_mutex: Mutex<i32>,
}

#[no_mangle]
pub static S1: Arc<st> = Arc::new(st {
    n1: 0,
    num_mutex: Mutex::new(0),
});

#[no_mangle]
pub static S2: Arc<st> = Arc::new(st {
    n1: 1,
    num_mutex: Mutex::new(0),
});

#[no_mangle]
pub static S3: Arc<st> = Arc::new(st {
    n1: 2,
    num_mutex: Mutex::new(0),
});

#[no_mangle]
pub extern "C" fn f(s: &Arc<st>) {
    let mut n1 = s.num_mutex.lock().unwrap();
    *n1 += 1;
}

#[no_mangle]
pub extern "C" fn f1() {
    f(&S1);
    f(&S2);
    f(&S3);
}

#[no_mangle]
pub extern "C" fn t_fun(_: *mut c_void) -> *mut c_void {
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

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            s1.n1,
            s2.n1,
            s3.n1,
        );
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}