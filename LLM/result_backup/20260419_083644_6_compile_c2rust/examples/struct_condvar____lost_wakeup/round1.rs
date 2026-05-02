use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut S: Arc<SharedData> = Arc::new(SharedData {
    n1: 0,
    m1: Mutex::new(()),
    cond: std::sync::Condvar::new(),
});

unsafe extern "C" fn f1() {
    let s = S.clone();
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

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, s.n1);
    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}