use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::Once;

#[derive(Debug)]
struct Ss1 {
    n1: i32,
    n2: i32,
    m1: Mutex<i32>,
}

#[derive(Debug)]
struct Ss2 {
    n1: i32,
    n3: i32,
    m2: Mutex<i32>,
}

lazy_static::lazy_static! {
    static ref S1: Arc<Mutex<Ss1>> = Arc::new(Mutex::new(Ss1 {
        n1: 0,
        n2: 1,
        m1: Mutex::new(0),
    }));
    static ref S2: Arc<Mutex<Ss2>> = Arc::new(Mutex::new(Ss2 {
        n1: 2,
        n3: 3,
        m2: Mutex::new(0),
    }));
}

static INIT: Once = Once::new();

fn init_globals() {
    // Initialization is done in lazy_static, no need for this function
}

unsafe extern "C" fn f1() {
    let x = {
        let s1 = S1.lock().unwrap();
        let s2 = S2.lock().unwrap();
        s1.n2 + s2.n3
    };

    let mut s1 = S1.lock().unwrap();
    let mut s2 = S2.lock().unwrap();
    s1.n1 += x;
    s2.n1 += x;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> c_int {
    INIT.call_once(init_globals);

    let handle1 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();

    let c_str1 = CString::new(format!("{} {}\n", s1.n1, s1.n2)).unwrap();
    let c_str2 = CString::new(format!("{} {}\n", s2.n1, s2.n3)).unwrap();

    unsafe {
        libc::printf(c_str1.as_ptr());
        libc::printf(c_str2.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
