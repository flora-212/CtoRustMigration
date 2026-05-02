use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use std::ptr;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

#[no_mangle]
pub static S1: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
}));

#[no_mangle]
pub static S2: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
}));

#[no_mangle]
pub static S3: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
}));

#[no_mangle]
pub fn h(s: &mut St) {
    s.n1 += 1;
}

#[no_mangle]
pub fn g(s: &mut St) {
    s.n1 += 1;
    h(s);
}

#[no_mangle]
pub fn f(s: &Arc<Mutex<St>>) {
    let mut s = s.lock().unwrap();
    s.n1 += 1;
    g(&mut s);
}

#[no_mangle]
pub fn f1() {
    f(&S1);
    f(&S2);
    f(&S3);
}

#[no_mangle]
pub fn t_fun(_: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> i32 {
    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);
    let s3 = Arc::clone(&S3);

    let id1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });
    let id2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let s3 = S3.lock().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const std::ffi::c_char,
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