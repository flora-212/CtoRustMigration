use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

#[derive(Debug)]
struct RwLock {
    lock: Mutex<()>,
    read_count: Mutex<usize>,
}

impl RwLock {
    fn new() -> Self {
        RwLock {
            lock: Mutex::new(()),
            read_count: Mutex::new(0),
        }
    }

    fn read_lock(&self) {
        let mut read_count = self.read_count.lock().unwrap();
        if *read_count == 0 {
            self.lock.lock().unwrap();
        }
        *read_count += 1;
    }

    fn read_unlock(&self) {
        let mut read_count = self.read_count.lock().unwrap();
        *read_count -= 1;
        if *read_count == 0 {
            drop(self.lock.lock().unwrap());
        }
    }

    fn write_lock(&self) {
        self.lock.lock().unwrap();
    }

    fn write_unlock(&self) {
        drop(self.lock.lock().unwrap());
    }
}

static LOCK: RwLock = RwLock::new();
static mut N: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn f1() -> c_int {
    LOCK.read_lock();
    let x = N;
    LOCK.read_unlock();
    x
}

#[no_mangle]
pub unsafe extern "C" fn f2() {
    LOCK.write_lock();
    N += 1;
    LOCK.write_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let c_string = CString::new(format!("{}\n", N)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}