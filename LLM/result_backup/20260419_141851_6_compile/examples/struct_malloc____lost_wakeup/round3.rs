use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[repr(C)]
pub struct ss {
    pub n: i32,
    pub m: Mutex<()>,
    pub c: std::sync::Condvar,
}

#[no_mangle]
pub extern "C" fn f1(s: *mut ss) {
    unsafe {
        let s = &mut *s;
        let mut guard = s.m.lock().unwrap();
        s.n += 1;
        guard = s.c.wait(guard).unwrap();
        drop(guard);
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1(arg as *mut ss);
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s = Arc::new(ss {
        n: 0,
        m: Mutex::new(()),
        c: std::sync::Condvar::new(),
    });

    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s1) as *mut ::core::ffi::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s2) as *mut ::core::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = Arc::try_unwrap(s).unwrap();
    println!("{}", s.n);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}