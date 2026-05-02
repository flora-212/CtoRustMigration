use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[repr(C)]
pub struct ss {
    pub n: i32,
    pub m: Mutex<()>,
}

#[no_mangle]
pub extern "C" fn inc(s: *mut ss) {
    unsafe {
        (*s).n += 1;
    }
}

#[no_mangle]
pub extern "C" fn f1(s: *mut ss) {
    unsafe {
        let mutex = &(*s).m;
        let _guard = mutex.lock().unwrap();
        inc(s);
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    unsafe {
        f1(arg as *mut ss);
    }
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s = Box::new(ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);

    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = s_ptr;
        let handle = thread::spawn(move || {
            unsafe {
                t_fun(s_clone as *mut std::ffi::c_void);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        let result = (*s_ptr).n;
        println!("{}", result);
        drop(Box::from_raw(s_ptr));
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}