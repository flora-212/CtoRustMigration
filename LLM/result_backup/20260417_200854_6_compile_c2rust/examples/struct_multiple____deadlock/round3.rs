use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;
use std::sync::OnceLock;

static INIT: Once = Once::new();
static S1: OnceLock<Arc<St>> = OnceLock::new();
static S2: OnceLock<Arc<St>> = OnceLock::new();
static S3: OnceLock<Arc<St>> = OnceLock::new();

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

fn get_s1() -> &'static Arc<St> {
    S1.get_or_init(|| Arc::new(St {
        n1: 0,
        num_mutex: Mutex::new(()),
    }))
}

fn get_s2() -> &'static Arc<St> {
    S2.get_or_init(|| Arc::new(St {
        n1: 1,
        num_mutex: Mutex::new(()),
    }))
}

fn get_s3() -> &'static Arc<St> {
    S3.get_or_init(|| Arc::new(St {
        n1: 2,
        num_mutex: Mutex::new(()),
    }))
}

#[no_mangle]
pub fn f(s: &Arc<St>, t: &Arc<St>) {
    let _guard_s = s.num_mutex.lock().unwrap();
    let _guard_t = t.num_mutex.lock().unwrap();
    s.n1 = t.n1 + 1;
    t.n1 = s.n1 + 1;
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

    println!("{} {} {}", get_s1().n1, get_s2().n1, get_s3().n1);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}