use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

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
        if *readers == 1 {
            self.lock.lock().unwrap();
        }
    }

    fn read_unlock(&self) {
        let mut readers = self.readers.lock().unwrap();
        *readers -= 1;
        if *readers == 0 {
            drop(self.lock.lock().unwrap());
        }
    }

    fn write(&self) {
        self.lock.lock().unwrap();
    }

    fn write_unlock(&self) {
        drop(self.lock.lock().unwrap());
    }
}

static LOCK: RwLock = RwLock::new();
static mut N: c_int = 0;

unsafe extern "C" fn f1() -> c_int {
    LOCK.read();
    let x = N;
    LOCK.read_unlock();
    x
}

unsafe extern "C" fn f2() {
    LOCK.write();
    N += 1;
    LOCK.write_unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
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

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}