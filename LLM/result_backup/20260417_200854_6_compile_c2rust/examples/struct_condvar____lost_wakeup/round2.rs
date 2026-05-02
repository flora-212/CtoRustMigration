use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct SharedState {
    n1: Mutex<i32>,
    m1: Mutex<()>,
    cond: Condvar,
}

static S: Arc<SharedState> = Arc::new(SharedState {
    n1: Mutex::new(0),
    m1: Mutex::new(()),
    cond: Condvar::new(),
});

unsafe extern "C" fn f1() {
    let s = S.clone();
    let mut guard = s.m1.lock().unwrap();
    *s.n1.lock().unwrap() += 1;
    if *s.n1.lock().unwrap() == 1 {
        guard = s.cond.wait(guard).unwrap();
    } else {
        guard = s.cond.wait(guard).unwrap();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *S.n1.lock().unwrap());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}