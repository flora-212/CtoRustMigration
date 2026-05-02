use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;

#[derive(Debug)]
struct Ss0 {
    n1: i32,
    n2: i32,
}

#[derive(Debug)]
struct Ss {
    s: Ss0,
    m: Mutex<()>,
}

lazy_static::lazy_static! {
    static ref S: Arc<Ss> = Arc::new(Ss {
        s: Ss0 { n1: 0, n2: 1 },
        m: Mutex::new(()),
    });
}

unsafe extern "C" fn f1() {
    let s = S.clone();
    let _guard = s.m.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = S.clone();
    let _guard = s.m.lock().unwrap();
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    let s = S.clone();
    let handle1 = thread::spawn(move || {
        let _guard = s.m.lock().unwrap();
        f1();
    });

    let s = S.clone();
    let handle2 = thread::spawn(move || {
        let _guard = s.m.lock().unwrap();
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.clone();
    let _guard = s.m.lock().unwrap();
    let c_str = CString::new(format!("{} {}\n", s.s.n1, s.s.n2)).unwrap();
    libc::printf(c_str.as_ptr());

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}