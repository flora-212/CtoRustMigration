use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ptr;
use std::ffi::CStr;
use libc;
use std::sync::Once;

#[derive(Debug)]
struct SharedState {
    n1: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static mut S: Option<Arc<SharedState>> = None;
static INIT: Once = Once::new();

fn initialize_shared_state() {
    let s = Arc::new(SharedState {
        n1: 0,
        m1: Mutex::new(()),
        cond: Condvar::new(),
    });
    unsafe {
        S = Some(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s = S.as_ref().unwrap().clone();
    let mut m1 = s.m1.lock().unwrap();
    s.n1 += 1;
    if s.n1 == 1 {
        m1 = s.cond.wait(m1).unwrap();
    } else {
        s.cond.notify_one();
    }
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(initialize_shared_state);

    let s1 = S.as_ref().unwrap().clone();
    let id1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let s2 = S.as_ref().unwrap().clone();
    let id2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let n1 = S.as_ref().unwrap().n1;
    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1);
    }
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}