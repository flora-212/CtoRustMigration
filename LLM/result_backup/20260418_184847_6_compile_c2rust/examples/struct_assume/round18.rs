use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

#[no_mangle]
pub extern "C" fn inc(s: &mut Ss) {
    s.n += 1;
}

#[no_mangle]
pub extern "C" fn f1(s: &mut Ss) {
    let _guard = s.m.lock().unwrap();
    inc(s);
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let s = unsafe { &mut *(arg as *mut Ss) };
    f1(s);
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });

    let s_clone1 = Arc::clone(&s);
    let s_clone2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        let s = Arc::as_ptr(&s_clone1);
        t_fun(s as *mut Ss);
    });

    let handle2 = thread::spawn(move || {
        let s = Arc::as_ptr(&s_clone2);
        t_fun(s as *mut Ss);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", s.n);
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}