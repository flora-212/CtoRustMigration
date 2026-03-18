use ::libc;
use std::ptr;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

pub struct ss {
    n: i32,
    m: Mutex<()>,
    c: Condvar,
}

unsafe extern "C" fn f1(s: *mut ss) {
    let s = &mut *s;
    let mut m = s.m.lock().unwrap();
    s.n += 1;
    if s.n == 1 {
        m = s.c.wait(m).unwrap();
    } else {
        s.c.notify_one();
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(arg as *mut ss);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s = Arc::new(ss {
        n: 0,
        m: Mutex::new(()),
        c: Condvar::new(),
    });

    let s1 = Arc::clone(&s);
    let id1 = thread::spawn(move || {
        unsafe { t_fun(s1 as *const _ as *mut libc::c_void) };
    });

    let s2 = Arc::clone(&s);
    let id2 = thread::spawn(move || {
        unsafe { t_fun(s2 as *const _ as *mut libc::c_void) };
    });

    id1.join().unwrap();
    id2.join().unwrap();

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}