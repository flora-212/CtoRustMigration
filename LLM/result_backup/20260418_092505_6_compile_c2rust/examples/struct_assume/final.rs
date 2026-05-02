use std::sync::{Arc, Mutex};
use std::thread;

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
    std::ptr::null_mut()
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
            t_fun(Arc::into_raw(s_clone) as *mut ::core::ffi::c_void);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", s.n);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
