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
    let s = arg as *mut Ss;
    unsafe {
        f1(&mut *s);
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = Arc::clone(&s);
        let handle = thread::spawn(move || {
            let s_ptr = Arc::into_raw(s_clone) as *mut Ss;
            t_fun(s_ptr);
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