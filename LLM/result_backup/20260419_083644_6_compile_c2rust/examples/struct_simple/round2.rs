use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

#[derive(Debug)]
pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<i32>,
    m2: Mutex<i32>,
}

static S: Arc<SharedData> = Arc::new(SharedData {
    n1: 0,
    n2: 1,
    n3: 2,
    n4: 3,
    m1: Mutex::new(0),
    m2: Mutex::new(0),
});

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s = &S;
    let x = s.n4;

    let mut guard1 = s.m1.lock().unwrap();
    *guard1 += x;

    let mut guard2 = s.m2.lock().unwrap();
    *guard2 += x;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s_clone1 = S.clone();
    let s_clone2 = S.clone();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = &S;
    let n1 = s.n1;
    let n2 = s.n2;
    let n3 = s.n3;
    let n4 = s.n4;

    let output = CString::new(format!("{} {} {} {}\n", n1, n2, n3, n4)).unwrap();
    libc::printf(output.as_ptr());

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}