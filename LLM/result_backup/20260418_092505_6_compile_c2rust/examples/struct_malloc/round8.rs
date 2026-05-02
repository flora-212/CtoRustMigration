use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[derive(Debug)]
struct S {
    n: i32,
    m: Mutex<()>,
    c: Condvar,
}

#[no_mangle]
pub extern "C" fn f1(s: &Arc<S>) {
    let mut m = s.m.lock().unwrap();
    s.n += 1;
    if s.n == 1 {
        m = s.c.wait(m).unwrap();
    } else {
        s.c.notify_one();
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let s = unsafe { Arc::from_raw(arg as *const S) };
    f1(&s);
    Arc::into_raw(s);
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s = Arc::new(S {
        n: 0,
        m: Mutex::new(()),
        c: Condvar::new(),
    });

    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);

    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s1) as *mut std::ffi::c_void);
    });

    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s2) as *mut std::ffi::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{}", s.n);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}