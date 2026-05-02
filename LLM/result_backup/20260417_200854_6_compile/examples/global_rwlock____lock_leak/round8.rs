use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use std::ptr;
use std::sync::Once;
use std::sync::OnceLock;

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

static LOCK: OnceLock<RwLock> = OnceLock::new();
static N: OnceLock<Arc<Mutex<i32>>> = OnceLock::new();

#[no_mangle]
pub extern "C" fn f1() -> i32 {
    LOCK.get().unwrap().read();
    *N.get().unwrap().lock().unwrap()
}

#[no_mangle]
pub extern "C" fn f2() {
    LOCK.get().unwrap().write();
    let mut n = N.get().unwrap().lock().unwrap();
    *n += 1;
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    ptr::null_mut()
}

fn main_0() -> i32 {
    LOCK.set(RwLock::new()).unwrap();
    N.set(Arc::new(Mutex::new(0))).unwrap();

    let lock = Arc::new(LOCK.get().unwrap().clone());
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

    let n = N.get().unwrap().lock().unwrap();
    unsafe {
        libc::printf(b"%d\n\0".as_ptr(), *n);
    }

    0
}

pub fn main() {
    std::process::exit(main_0() as i32);
}