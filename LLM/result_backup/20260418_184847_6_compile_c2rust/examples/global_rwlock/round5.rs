use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::c_void;

#[derive(Debug)]
pub struct RwLock {
    inner: Arc<Mutex<i32>>,
}

impl RwLock {
    pub fn new(value: i32) -> Self {
        RwLock {
            inner: Arc::new(Mutex::new(value)),
        }
    }

    pub fn read(&self) -> i32 {
        *self.inner.lock().unwrap()
    }

    pub fn write(&self, value: i32) {
        *self.inner.lock().unwrap() = value;
    }
}

lazy_static::lazy_static! {
    pub static ref LOCK: RwLock = RwLock::new(0);
}

#[no_mangle]
pub extern "C" fn f1() -> i32 {
    LOCK.read()
}

#[no_mangle]
pub extern "C" fn f2() {
    LOCK.write(LOCK.read() + 1);
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let lock = LOCK.inner.clone();

    let handle1 = thread::spawn(move || {
        let _ = t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        let _ = t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = LOCK.read();
    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}