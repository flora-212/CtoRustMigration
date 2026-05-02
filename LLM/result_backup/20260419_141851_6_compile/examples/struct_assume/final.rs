use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

#[no_mangle]
pub extern "C" fn inc(s: *mut Ss) {
    unsafe {
        (*s).n += 1;
    }
}

#[no_mangle]
pub extern "C" fn f1(s: *mut Ss) {
    unsafe {
        let guard = (*s).m.lock().unwrap();
        inc(s);
        drop(guard);
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut Ss) -> *mut std::ffi::c_void {
    unsafe {
        f1(arg);
    }
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });

    let s_clone1 = Arc::clone(&s);
    let s_clone2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone1) as *mut Ss);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone2) as *mut Ss);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", s.n);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
