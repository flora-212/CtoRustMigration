use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

#[repr(C)]
pub struct ss {
    pub n: i32,
    pub m: Mutex<()>,
}

#[no_mangle]
pub static mut x: *mut ss = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s = Box::new(ss {
        n: 123,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);
    x = s_ptr;
    let mut guard = (*x).m.lock().unwrap();
    (*x).n = 456;
    drop(guard);
}

#[no_mangle]
pub unsafe extern "C" fn f2() {
    let s = Box::new(ss {
        n: 123,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);
    x = s_ptr;
    let mut guard = (*x).m.lock().unwrap();
    (*x).n = 789;
    drop(guard);
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });
    let handle2 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    if !x.is_null() {
        let n = (*x).n;
        libc::printf(b"%d\n\0".as_ptr(), n);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}