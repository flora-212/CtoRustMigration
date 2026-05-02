use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::sync::Once;

#[derive(Debug)]
struct Ss1 {
    n1: c_int,
    n2: c_int,
    m1: Mutex<()>,
}

#[derive(Debug)]
struct Ss2 {
    n1: c_int,
    n3: c_int,
    m2: Mutex<()>,
}

lazy_static::lazy_static! {
    static ref S1: Arc<Mutex<Ss1>> = Arc::new(Mutex::new(Ss1 {
        n1: 0,
        n2: 1,
        m1: Mutex::new(()),
    }));
    static ref S2: Arc<Mutex<Ss2>> = Arc::new(Mutex::new(Ss2 {
        n1: 2,
        n3: 3,
        m2: Mutex::new(()),
    }));
}

static INIT: Once = Once::new();

fn init_globals() {
    INIT.call_once(|| {
        // Initialization code if needed
    });
}

unsafe extern "C" fn f1() {
    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let x = s1.n2 + s2.n3;
    drop(s1);
    drop(s2);

    let mut s1 = S1.lock().unwrap();
    let mut s2 = S2.lock().unwrap();
    s1.n1 += x;
    s2.n1 += x;
}

unsafe extern "C" fn f2() {
    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let x = s1.n2 + s2.n3;
    drop(s1);
    drop(s2);

    let mut s1 = S1.lock().unwrap();
    let mut s2 = S2.lock().unwrap();
    s1.n1 += x;
    s2.n1 += x;
}

unsafe extern "C" fn t_fun(arg: *mut c_void) -> *mut c_void {
    if arg as *mut c_int as usize == 0 {
        f1();
    } else {
        f2();
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    init_globals();

    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(1 as *mut c_void);
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
