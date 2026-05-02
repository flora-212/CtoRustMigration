use std::sync::{Arc, Mutex};
use std::thread;

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

    fn unlock(&self) {
        // No-op in this implementation
    }
}

static LOCK: RwLock = RwLock::new();
static mut N: i32 = 0;

#[no_mangle]
pub extern "C" fn f1() -> i32 {
    LOCK.read();
    let x = unsafe { N };
    LOCK.unlock();
    x
}

#[no_mangle]
pub extern "C" fn f2() {
    LOCK.write();
    unsafe { N += 1 };
    LOCK.unlock();
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let lock = Arc::new(LOCK);
    let lock_clone = lock.clone();

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = unsafe { N };
    unsafe { libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n) };
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}