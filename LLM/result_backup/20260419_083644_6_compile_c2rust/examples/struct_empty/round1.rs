use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[repr(C)]
pub struct ss {
    pub n: i32,
    pub m: Mutex<()>,
}

#[no_mangle]
pub static mut s1: ss = ss {
    n: 0,
    m: Mutex::new(()),
};

#[no_mangle]
pub unsafe extern "C" fn f1(s: *mut ss) {
    let s = &mut *s;
    let _guard = s.m.lock().unwrap();
    // Simulate some work
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(&mut s1 as *mut ss);
    f1(arg as *mut ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Box::new(ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);

    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = s_ptr.clone();
        let handle = thread::spawn(move || {
            unsafe {
                t_fun(s_clone as *mut libc::c_void);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let s = unsafe { Box::from_raw(s_ptr) };
    libc::printf(b"%d %d\n\0".as_ptr() as *const libc::c_char, s.n, s1.n);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}