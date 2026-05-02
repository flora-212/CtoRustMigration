use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct S {
    n: i32,
    m: Mutex<()>,
    c: std::sync::Condvar,
}

#[no_mangle]
pub extern "C" fn f1(s: Arc<S>) {
    let mut guard = s.m.lock().unwrap();
    s.n += 1;
    guard = s.c.wait(guard).unwrap();
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let s = unsafe { Arc::from_raw(arg as *const S) };
    f1(s);
    ptr::null_mut()
}

fn main_0() -> i32 {
    let s = Arc::new(S {
        n: 0,
        m: Mutex::new(()),
        c: std::sync::Condvar::new(),
    });

    let s_clone1 = Arc::clone(&s);
    let s_clone2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s_clone1) as *mut ::core::ffi::c_void) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s_clone2) as *mut ::core::ffi::c_void) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", s.n);

    0
}

pub fn main() {
    std::process::exit(main_0());
}