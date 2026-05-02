use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[repr(C)]
pub struct ss {
    pub n: i32,
    pub m: Mutex<i32>,
}

#[no_mangle]
pub static mut x: *mut ss = ptr::null_mut();

#[no_mangle]
pub extern "C" fn f1() {
    let s = Box::new(ss {
        n: 123,
        m: Mutex::new(0),
    });
    unsafe {
        x = Box::into_raw(s);
    }
    let mut m = unsafe { (*x).m.lock().unwrap() };
    *m = 456;
}

#[no_mangle]
pub extern "C" fn f2() {
    let s = Box::new(ss {
        n: 789,
        m: Mutex::new(0),
    });
    unsafe {
        x = Box::into_raw(s);
    }
    let mut m = unsafe { (*x).m.lock().unwrap() };
    *m = 789;
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
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

    let n = unsafe { (*x).n };
    println!("{}", n);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
