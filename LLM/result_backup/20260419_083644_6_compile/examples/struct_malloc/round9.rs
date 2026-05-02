use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[derive(Debug)]
struct S {
    n: i32,
    m: Mutex<()>,
    c: std::sync::Condvar,
}

#[no_mangle]
pub extern "C" fn f1(s: *mut S) {
    let s = unsafe { &mut *s };
    let mut guard = s.m.lock().unwrap();
    s.n += 1;
    if s.n == 1 {
        s.c.wait(&mut guard);
    } else {
        s.c.notify_one();
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut S) -> *mut S {
    f1(arg);
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s = Arc::new(S {
        n: 0,
        m: Mutex::new(()),
        c: std::sync::Condvar::new(),
    });

    let s_clone1 = Arc::clone(&s);
    let s_clone2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone1) as *mut S);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone2) as *mut S);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = Arc::try_unwrap(s).unwrap();
    println!("{}", s.n);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}