use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ffi::CString;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<i32>,
    cond: Condvar,
}

lazy_static::lazy_static! {
    static ref S: Arc<SharedData> = Arc::new(SharedData {
        n1: 0,
        m1: Mutex::new(0),
        cond: Condvar::new(),
    });
}

unsafe extern "C" fn f1() {
    let s = S.clone();
    let mut guard = s.m1.lock().unwrap();
    *guard += 1;
    if *guard == 1 {
        guard = s.cond.wait(guard).unwrap();
    } else {
        guard = s.cond.wait(guard).unwrap();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = S.clone();
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(move || {
            f1();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let c_string = CString::new(format!("{}\n", s.n1)).unwrap();
    libc::printf(c_string.as_ptr());
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
