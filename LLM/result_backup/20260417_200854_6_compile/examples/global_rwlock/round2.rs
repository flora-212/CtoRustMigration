use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CStr;
use libc;

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
        let mut readers = self.readers.lock().unwrap();
        *readers += 1;
    }

    fn read_unlock(&self) {
        let mut readers = self.readers.lock().unwrap();
        *readers -= 1;
    }

    fn write(&self) {
        self.lock.lock().unwrap();
    }

    fn write_unlock(&self) {
        self.lock.unlock();
    }
}

static LOCK: RwLock = RwLock::new();
static mut N: i32 = 0;

#[no_mangle]
pub unsafe extern "C" fn f1() -> i32 {
    LOCK.read();
    let x = N;
    LOCK.read_unlock();
    x
}

#[no_mangle]
pub unsafe extern "C" fn f2() {
    LOCK.write();
    N += 1;
    LOCK.write_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let handle1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });
    let handle2 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N);
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}