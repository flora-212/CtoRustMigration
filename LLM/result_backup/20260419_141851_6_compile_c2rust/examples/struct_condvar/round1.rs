use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedState {
    n1: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut S: Option<Arc<SharedState>> = None;

fn init_shared_state() -> Arc<SharedState> {
    unsafe {
        if S.is_none() {
            S = Some(Arc::new(SharedState {
                n1: 0,
                m1: Mutex::new(()),
                cond: std::sync::Condvar::new(),
            }));
        }
        S.as_ref().unwrap().clone()
    }
}

unsafe extern "C" fn f1() {
    let s = init_shared_state();
    let mut guard = s.m1.lock().unwrap();
    s.n1 += 1;
    if s.n1 == 1 {
        guard = s.cond.wait(guard).unwrap();
    } else {
        s.cond.notify_one();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s = init_shared_state();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, s.n1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}