use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ptr;
use std::ffi::CString;
use std::sync::OnceLock;

struct SharedState {
    n1: Mutex<i32>,
    m1: Mutex<()>,
    cond: Condvar,
}

static S: OnceLock<Arc<SharedState>> = OnceLock::new();

unsafe extern "C" fn f1() {
    let s = S.get().unwrap().clone();
    let mut guard = s.m1.lock().unwrap();
    let mut n1_guard = s.n1.lock().unwrap();
    *n1_guard += 1;
    if *n1_guard == 1 {
        guard = s.cond.wait(guard).unwrap();
    } else {
        s.cond.notify_one();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    S.get_or_init(|| Arc::new(SharedState {
        n1: Mutex::new(0),
        m1: Mutex::new(()),
        cond: Condvar::new(),
    }));

    let s = S.get().unwrap().clone();
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

    let n1 = s.n1.lock().unwrap();
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *n1);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}