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
        let mutex = (*s).m.lock().unwrap();
        inc(s);
        drop(mutex);
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1(arg as *mut ss);
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
                t_fun(s_clone as *mut ::core::ffi::c_void);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        let s = Box::from_raw(s_ptr);
        println!("{}", s.n);
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}