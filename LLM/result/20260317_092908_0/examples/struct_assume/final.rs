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
    let s = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = Arc::clone(&s);
        let handle = thread::spawn(move || {
            let s = Arc::as_ptr(&s_clone) as *mut ss;
            t_fun(s);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
