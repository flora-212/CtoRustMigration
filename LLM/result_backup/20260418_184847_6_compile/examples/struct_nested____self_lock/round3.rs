use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use libc;

#[derive(Debug)]
pub struct Ss0 {
    pub n1: i32,
    pub n2: i32,
}

#[derive(Debug)]
pub struct Ss {
    pub s: Ss0,
    pub m: Mutex<()>,
}

#[no_mangle]
pub static S: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
    s: Ss0 { n1: 0, n2: 1 },
    m: Mutex::new(()),
}));

#[no_mangle]
pub extern "C" fn f1() {
    let s = S.clone();
    let mut s = s.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = S.clone();
    let _guard = s.lock().unwrap();
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = S.clone();
    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let s = S.clone();
    let handle2 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.lock().unwrap();
    let output = format!("{} {}\n", s.s.n1, s.s.n2);
    let c_string = CString::new(output).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}