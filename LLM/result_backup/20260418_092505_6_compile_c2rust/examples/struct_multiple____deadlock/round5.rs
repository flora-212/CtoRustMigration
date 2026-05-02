use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;

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
pub fn f(s: &Arc<Mutex<St>>, t: &Arc<Mutex<St>>) {
    let mut s = s.lock().unwrap();
    let mut t = t.lock().unwrap();
    s.n1 = t.n1 + 1;
    t.n1 = s.n1 + 1;
}

#[no_mangle]
pub fn f1() {
    f(&S1, &S2);
}

#[no_mangle]
pub fn f2() {
    f(&S2, &S3);
}

#[no_mangle]
pub fn f3() {
    f(&S1, &S3);
}

#[no_mangle]
pub fn t_fun(arg: *mut libc::c_void) {
    if arg as libc::c_long == 0 {
        f1();
    } else if arg as libc::c_long == 1 {
        f2();
    } else {
        f3();
    }
}

unsafe fn main_0() -> libc::c_int {
    let id1 = thread::spawn(|| t_fun(0 as *mut libc::c_void));
    let id2 = thread::spawn(|| t_fun(1 as *mut libc::c_void));
    let id3 = thread::spawn(|| t_fun(2 as *mut libc::c_void));

    id1.join().unwrap();
    id2.join().unwrap();
    id3.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let s3 = S3.lock().unwrap();

    let message = format!("{} {} {}\n", s1.n1, s2.n1, s3.n1);
    let c_string = CString::new(message).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}