use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct RwLock {
    lock: Mutex<()>,
    readers: Mutex<usize>,
}

impl RwLock {
    fn new() -> Self {
        RwLock {
            lock: Mutex::new(()),
            readers: Mutex::new(0),
        }
    }

    fn read(&self) {
        let _readers = self.readers.lock().unwrap();
        let _lock = self.lock.lock().unwrap();
    }

    fn write(&self) {
        let _lock = self.lock.lock().unwrap();
    }
}

lazy_static::lazy_static! {
    static ref LOCK: RwLock = RwLock::new();
    static ref N: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

#[no_mangle]
pub extern "C" fn f1() -> i32 {
    LOCK.read();
    *N.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn f2() {
    LOCK.write();
    let mut n = N.lock().unwrap();
    *n += 1;
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    ptr::null_mut()
}

fn main_0() -> i32 {
    let lock = Arc::new(LOCK);
    let mut handles = vec![];

    for _ in 0..2 {
        let lock_clone = lock.clone();
        let handle = thread::spawn(move || {
            t_fun(ptr::null_mut());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let n = N.lock().unwrap();
    unsafe {
        libc::printf(b"%d\n\0".as_ptr(), *n);
    }

    0
}

pub fn main() {
    std::process::exit(main_0() as i32);
}