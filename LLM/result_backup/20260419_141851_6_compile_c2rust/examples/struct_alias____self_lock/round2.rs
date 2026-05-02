use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

#[no_mangle]
pub static mut S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static mut S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static mut S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub unsafe extern "C" fn h(u: *mut St) {
    let u = &mut *u;
    let _guard = u.num_mutex.lock().unwrap();
    u.n1 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn g(t: *mut St) {
    let t = &mut *t;
    t.n1 += 1;
    h(t);
}

#[no_mangle]
pub unsafe extern "C" fn f(s: *mut St) {
    let s = &mut *s;
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    f(&mut *S1.clone().as_ptr());
    f(&mut *S2.clone().as_ptr());
    f(&mut *S3.clone().as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);
    let s3 = Arc::clone(&S3);

    let handle1 = thread::spawn(move || {
        f(&mut *s1.as_ptr());
    });

    let handle2 = thread::spawn(move || {
        f(&mut *s2.as_ptr());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);
    let s3 = Arc::clone(&S3);

    let message = format!("{} {} {}\n", s1.n1, s2.n1, s3.n1);
    let c_string = CString::new(message).unwrap();
    libc::printf(c_string.as_ptr());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}