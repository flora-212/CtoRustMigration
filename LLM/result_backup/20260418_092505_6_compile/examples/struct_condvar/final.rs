use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::sync::ONCE_INIT;

static mut S: Option<Arc<SharedState>> = None;
static INIT: Once = ONCE_INIT;

struct SharedState {
    n1: Mutex<i32>,
    m1: Mutex<()>,
    cond: Condvar,
}

fn get_shared_state() -> Arc<SharedState> {
    unsafe {
        INIT.call_once(|| {
            S = Some(Arc::new(SharedState {
                n1: Mutex::new(0),
                m1: Mutex::new(()),
                cond: Condvar::new(),
            }));
        });
        S.as_ref().unwrap().clone()
    }
}

unsafe extern "C" fn f1() {
    let s = get_shared_state();
    let mut guard = s.m1.lock().unwrap();
    let mut n1 = s.n1.lock().unwrap();
    *n1 += 1;
    if *n1 == 1 {
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
    let s = get_shared_state();
    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = s.clone();
        let handle = thread::spawn(move || {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *s.n1.lock().unwrap());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
