use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::Once;

static INIT: Once = Once::new();

struct Ss1 {
    n1: c_int,
    n2: c_int,
    m1: Mutex<c_int>,
}

struct Ss2 {
    n1: c_int,
    n3: c_int,
    m2: Mutex<c_int>,
}

static mut S1: Option<Arc<Ss1>> = None;
static mut S2: Option<Arc<Ss2>> = None;

fn get_s1() -> Arc<Ss1> {
    INIT.call_once(|| {
        let s1 = Arc::new(Ss1 {
            n1: 0,
            n2: 1,
            m1: Mutex::new(0),
        });
        unsafe {
            S1 = Some(s1);
        }
    });
    unsafe { S1.as_ref().unwrap().clone() }
}

fn get_s2() -> Arc<Ss2> {
    INIT.call_once(|| {
        let s2 = Arc::new(Ss2 {
            n1: 2,
            n3: 3,
            m2: Mutex::new(0),
        });
        unsafe {
            S2 = Some(s2);
        }
    });
    unsafe { S2.as_ref().unwrap().clone() }
}

unsafe extern "C" fn f1() {
    let s1 = get_s1();
    let s2 = get_s2();
    let x = s1.n2 + s2.n3;
    let mut guard1 = s1.m1.lock().unwrap();
    let mut guard2 = s2.m2.lock().unwrap();
    *guard1 += x;
    *guard2 += x;
}

unsafe extern "C" fn f2() {
    let s1 = get_s1();
    let s2 = get_s2();
    let x = s1.n2 + s2.n3;
    let mut guard2 = s2.m2.lock().unwrap();
    let mut guard1 = s1.m1.lock().unwrap();
    *guard2 += x;
    *guard1 += x;
}

unsafe extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    if arg as c_int == 0 {
        f1();
    } else {
        f2();
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    let s1 = get_s1();
    let s2 = get_s2();

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(1 as *mut std::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = get_s1();
    let s2 = get_s2();

    let cstr1 = CString::new(format!("{} {}\n", s1.n1, s1.n2)).unwrap();
    let cstr2 = CString::new(format!("{} {}\n", s2.n1, s2.n3)).unwrap();

    unsafe {
        libc::printf(cstr1.as_ptr());
        libc::printf(cstr2.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}