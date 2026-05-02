use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use std::ptr;
use std::sync::Once;
use std::sync::OnceLock;

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

static NUM_MUTEX: OnceLock<Arc<Mutex<i32>>> = OnceLock::new();

#[no_mangle]
pub extern "C" fn lock() {
    NUM_MUTEX.get().unwrap().lock().unwrap();
}

#[no_mangle]
pub extern "C" fn unlock() {
    // No-op in Rust, as the lock is released when the guard goes out of scope
}

#[no_mangle]
pub extern "C" fn f1() {
    let mut n1 = NUM_MUTEX.get().unwrap().lock().unwrap();
    *n1 += 1;
}

#[no_mangle]
pub extern "C" fn lock2(n: i32) -> i32 {
    let mut n1 = NUM_MUTEX.get().unwrap().lock().unwrap();
    *n1 += n;
    *n1
}

#[no_mangle]
pub extern "C" fn unlock2(n: i32) -> i32 {
    let mut n1 = NUM_MUTEX.get().unwrap().lock().unwrap();
    *n1 += n;
    *n1
}

#[no_mangle]
pub extern "C" fn f2() -> i32 {
    let n2 = lock2(1);
    let mut n1 = NUM_MUTEX.get().unwrap().lock().unwrap();
    *n1 += 1;
    n2 + *n1
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    NUM_MUTEX.get_or_init(|| Arc::new(Mutex::new(0)));

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

    let n1 = NUM_MUTEX.get().unwrap().lock().unwrap();
    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *n1);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
