use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedState {
    n1: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static mut S: Option<Arc<SharedState>> = None;

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s = S.as_ref().unwrap();
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
    let s = Arc::new(SharedState {
        n1: 0,
        m1: Mutex::new(()),
        cond: Condvar::new(),
    });
    S = Some(s.clone());

    let s1 = s.clone();
    let id1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let s2 = s.clone();
    let id2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s = S.as_ref().unwrap();
    let n1 = s.n1;
    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1);
    }
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}