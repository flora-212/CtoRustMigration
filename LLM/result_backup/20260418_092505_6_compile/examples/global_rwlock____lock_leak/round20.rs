use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

#[derive(Debug)]
pub struct RwLock {
    lock: Arc<Mutex<i32>>,
}

impl RwLock {
    pub fn new(value: i32) -> Self {
        RwLock {
            lock: Arc::new(Mutex::new(value)),
        }
    }

    pub fn read(&self) -> i32 {
        let lock = self.lock.lock().unwrap();
        *lock
    }

    pub fn write(&self) {
        let mut lock = self.lock.lock().unwrap();
        *lock += 1;
    }
}

#[no_mangle]
pub static rw_lock: RwLock = RwLock::new(0);

#[no_mangle]
pub extern "C" fn f1() -> i32 {
    rw_lock.read()
}

#[no_mangle]
pub extern "C" fn f2() {
    rw_lock.write()
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let rw_lock = Arc::new(rw_lock.lock.clone());

    let handle1 = thread::spawn(move || {
        let _ = t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        let _ = t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = rw_lock.lock.lock().unwrap();
    println!("{}", *n);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}