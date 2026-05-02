use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss0 {
    pub n1: i32,
    pub n2: i32,
}

#[derive(Clone)]
pub struct ss {
    pub s: ss0,
    pub m: Arc<Mutex<()>>,
}

#[no_mangle]
pub static S: Arc<ss> = Arc::new(ss {
    s: ss0 { n1: 0, n2: 1 },
    m: Arc::new(Mutex::new(())),
});

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut s = S.clone();
    let _guard = s.m.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s_clone = Arc::clone(&S);
    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    let s_clone = Arc::clone(&S);
    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let c_string = CString::new(format!("{} {}\n", S.s.n1, S.s.n2)).unwrap();
    libc::printf(c_string.as_ptr());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}