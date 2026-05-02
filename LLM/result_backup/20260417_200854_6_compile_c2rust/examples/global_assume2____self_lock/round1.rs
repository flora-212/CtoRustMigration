use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
pub struct PthreadMutex {
    inner: Mutex<()>,
}

impl PthreadMutex {
    pub fn lock(&self) {
        self.inner.lock().unwrap();
    }

    pub fn unlock(&self) {
        // No-op in Rust's Mutex, as unlock is implicit when the guard is dropped
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    inner: Mutex::new(()),
};

static mut N1: i32 = 0;

unsafe extern "C" fn inc() -> i32 {
    N1 += 1;
    if N1 != 0 {
        N1
    } else {
        N1 + 1
    }
}

unsafe extern "C" fn f1() {
    NUM_MUTEX.lock();
    NUM_MUTEX.lock();
    inc();
    NUM_MUTEX.unlock();
    NUM_MUTEX.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> i32 {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}