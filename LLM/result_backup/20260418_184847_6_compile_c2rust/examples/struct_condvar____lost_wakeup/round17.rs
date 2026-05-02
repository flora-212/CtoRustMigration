use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use libc;

#[derive(Debug)]
struct SharedState {
    n1: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut S: Option<Arc<SharedState>> = None;

static INIT: Once = Once::new();

fn initialize_shared_state() {
    unsafe {
        S = Some(Arc::new(SharedState {
            n1: 0,
            m1: Mutex::new(()),
            cond: std::sync::Condvar::new(),
        }));
    }
}

unsafe extern "C" fn f1() {
    let s = S.as_ref().unwrap().clone();
    let mut guard = s.m1.lock().unwrap();
    s.n1 += 1;
    if s.n1 == 1 {
        guard = s.cond.wait(guard).unwrap();
    } else {
        guard = s.cond.wait(guard).unwrap();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(initialize_shared_state);

    let s = S.as_ref().unwrap().clone();
    let mut id1 = None;
    let mut id2 = None;

    id1 = Some(thread::spawn(move || {
        t_fun(ptr::null_mut());
    }));

    id2 = Some(thread::spawn(move || {
        t_fun(ptr::null_mut());
    }));

    id1.take().unwrap().join().unwrap();
    id2.take().unwrap().join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, S.as_ref().unwrap().n1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}