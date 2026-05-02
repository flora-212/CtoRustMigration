use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

lazy_static! {
    #[no_mangle]
    pub static ref S1: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
        n1: 0,
        num_mutex: Mutex::new(()),
    }));

    #[no_mangle]
    pub static ref S2: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
        n1: 1,
        num_mutex: Mutex::new(()),
    }));

    #[no_mangle]
    pub static ref S3: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
        n1: 2,
        num_mutex: Mutex::new(()),
    }));
}

#[no_mangle]
pub unsafe extern "C" fn h(s: &mut St) {
    s.n1 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn g(s: &mut St) {
    s.n1 += 1;
    h(s);
}

#[no_mangle]
pub unsafe extern "C" fn f(s: &mut St) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut s1 = S1.lock().unwrap();
    let mut s2 = S2.lock().unwrap();
    let mut s3 = S3.lock().unwrap();
    f(&mut s1);
    f(&mut s2);
    f(&mut s3);
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        f1();
    });

    let handle2 = thread::spawn(|| {
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1_n1 = S1.lock().unwrap().n1;
    let s2_n1 = S2.lock().unwrap().n1;
    let s3_n1 = S3.lock().unwrap().n1;

    let message = CString::new(format!("{} {} {}\n", s1_n1, s2_n1, s3_n1)).unwrap();
    libc::printf(message.as_ptr());

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}