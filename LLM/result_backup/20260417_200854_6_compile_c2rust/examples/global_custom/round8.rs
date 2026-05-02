use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    fn new() -> Self {
        PthreadMutex {
            mutex: Mutex::new(()),
        }
    }

    fn lock(&self) {
        let _guard = self.mutex.lock().unwrap();
    }

    fn unlock(&self) {
        // No-op in Rust, as the lock is released when the guard goes out of scope
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex::new();
static mut N1: i32 = 0;

#[no_mangle]
pub extern "C" fn lock() {
    NUM_MUTEX.lock();
}

#[no_mangle]
pub extern "C" fn unlock() {
    NUM_MUTEX.unlock();
}

#[no_mangle]
pub extern "C" fn f1() {
    lock();
    unsafe {
        N1 += 1;
    }
    unlock();
}

#[no_mangle]
pub extern "C" fn lock2(n: i32) -> i32 {
    NUM_MUTEX.lock();
    unsafe {
        N1 += n;
    }
    N1
}

#[no_mangle]
pub extern "C" fn unlock2(n: i32) -> i32 {
    unsafe {
        N1 += n;
    }
    let n2 = N1;
    NUM_MUTEX.unlock();
    n2
}

#[no_mangle]
pub extern "C" fn f2() -> i32 {
    let n2 = lock2(1);
    unsafe {
        N1 += 1;
    }
    let n2 = n2 + unlock2(1);
    n2
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            unsafe {
                t_fun(ptr::null_mut());
            }
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

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}