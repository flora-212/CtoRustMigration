use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;
use std::sync::OnceLock;

static INIT: Once = Once::new();
static S1: OnceLock<Arc<Mutex<St>>> = OnceLock::new();
static S2: OnceLock<Arc<Mutex<St>>> = OnceLock::new();
static S3: OnceLock<Arc<Mutex<St>>> = OnceLock::new();

#[derive(Debug)]
pub struct St {
    pub n1: i32,
}

fn get_s1() -> &'static Arc<Mutex<St>> {
    S1.get_or_init(|| Arc::new(Mutex::new(St { n1: 0 })))
}

fn get_s2() -> &'static Arc<Mutex<St>> {
    S2.get_or_init(|| Arc::new(Mutex::new(St { n1: 1 })))
}

fn get_s3() -> &'static Arc<Mutex<St>> {
    S3.get_or_init(|| Arc::new(Mutex::new(St { n1: 2 })))
}

#[no_mangle]
pub fn f(s: &Arc<Mutex<St>>, t: &Arc<Mutex<St>>) {
    let mut s_guard = s.lock().unwrap();
    let mut t_guard = t.lock().unwrap();
    s_guard.n1 = t_guard.n1 + 1;
    t_guard.n1 = s_guard.n1 + 1;
}

#[no_mangle]
pub fn f1() {
    f(get_s1(), get_s2());
}

#[no_mangle]
pub fn f2() {
    f(get_s2(), get_s3());
}

#[no_mangle]
pub fn f3() {
    f(get_s1(), get_s3());
}

#[no_mangle]
pub fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    if arg as isize == 0 {
        f1();
    } else if arg as isize == 1 {
        f2();
    } else {
        f3();
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    INIT.call_once(|| {
        // Initialization code if needed
    });

    let s1 = Arc::clone(get_s1());
    let s2 = Arc::clone(get_s2());
    let s3 = Arc::clone(get_s3());

    let handle1 = thread::spawn(move || {
        t_fun(2 as *mut std::ffi::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(1 as *mut std::ffi::c_void);
    });

    let handle3 = thread::spawn(move || {
        t_fun(3 as *mut std::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    let s1 = get_s1().lock().unwrap();
    let s2 = get_s2().lock().unwrap();
    let s3 = get_s3().lock().unwrap();
    println!("{} {} {}", s1.n1, s2.n1, s3.n1);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}