use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

#[repr(C)]
pub struct ss1 {
    pub n1: c_int,
    pub n2: c_int,
    pub m1: Arc<Mutex<()>>,
}

#[repr(C)]
pub struct ss2 {
    pub n1: c_int,
    pub n3: c_int,
    pub m2: Arc<Mutex<()>>,
}

lazy_static::lazy_static! {
    #[no_mangle]
    pub static ref S1: Arc<Mutex<ss1>> = Arc::new(Mutex::new(ss1 {
        n1: 0,
        n2: 1,
        m1: Arc::new(Mutex::new(())),
    }));

    #[no_mangle]
    pub static ref S2: Arc<Mutex<ss2>> = Arc::new(Mutex::new(ss2 {
        n1: 2,
        n3: 3,
        m2: Arc::new(Mutex::new(())),
    }));
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let mut x: c_int = s1.n2 + s2.n3;
    let _guard1 = s1.m1.lock().unwrap();
    let _guard2 = s2.m2.lock().unwrap();
    let mut s1_mut = S1.lock().unwrap();
    let mut s2_mut = S2.lock().unwrap();
    s1_mut.n1 = s1_mut.n1 + x;
    s2_mut.n1 = s2_mut.n1 + x;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut c_void) -> *mut c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    let t1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let t2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();

    let c_str1 = CString::new(format!("{} {}\n", s1.n1, s1.n2)).unwrap();
    let c_str2 = CString::new(format!("{} {}\n", s2.n1, s2.n3)).unwrap();

    libc::printf(c_str1.as_ptr());
    libc::printf(c_str2.as_ptr());

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}