use ::libc;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
#[repr(C)]
pub struct ss {
    pub n: libc::c_int,
    pub m: Arc<Mutex<()>>,
}

pub fn inc(s: &mut ss) {
    s.n += 1;
}

pub fn f1(s: &mut ss) {
    let _guard = s.m.lock().unwrap();
    inc(s);
}

pub extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = unsafe { &mut *(arg as *mut ss) };
    f1(s);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut s = Box::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });
    let s_clone = s.clone();

    let handle1 = thread::spawn(move || {
        t_fun(Box::into_raw(s) as *mut libc::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Box::into_raw(s_clone) as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}