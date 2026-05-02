use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::ptr;

#[derive(Debug)]
struct Ss0 {
    n1: i32,
    n2: i32,
}

#[derive(Debug)]
struct Ss {
    s: Mutex<Ss0>,
}

lazy_static::lazy_static! {
    static ref S: Arc<Ss> = Arc::new(Ss {
        s: Mutex::new(Ss0 { n1: 0, n2: 1 }),
    });
}

unsafe extern "C" fn f1() {
    let s = S.clone();
    let mut s = s.s.lock().unwrap();
    s.n1 += 1;
    s.n2 += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s = S.clone();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.clone();
    let s = s.s.lock().unwrap();
    let n1 = s.n1;
    let n2 = s.n2;

    let c_string = CString::new(format!("{} {}\n", n1, n2)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}