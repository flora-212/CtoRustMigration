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
    let s = Arc::new(ss {
        n: 0,
        m: Mutex::new(()),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = Arc::clone(&s);
        let handle = thread::spawn(move || {
            let s_ptr = Arc::into_raw(s_clone);
            unsafe {
                t_fun(s_ptr as *mut std::ffi::c_void);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        let result = (*Arc::as_ptr(&s)).n;
        println!("{}", result);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
