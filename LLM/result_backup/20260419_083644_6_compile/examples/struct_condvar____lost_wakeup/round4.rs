use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ptr;
use std::ffi::c_void;
use std::sync::Once;
use std::sync::OnceLock;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static S: OnceLock<Arc<SharedData>> = OnceLock::new();

unsafe extern "C" fn f1() {
    let s = S.get().unwrap().clone();
    let mut guard = s.m1.lock().unwrap();
    s.n1 += 1;
    if s.n1 == 1 {
        guard = s.cond.wait(guard).unwrap();
    } else {
        guard = s.cond.wait(guard).unwrap();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    S.get_or_init(|| Arc::new(SharedData {
        n1: 0,
        m1: Mutex::new(()),
        cond: Condvar::new(),
    }));

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

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, S.get().unwrap().n1);
    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}