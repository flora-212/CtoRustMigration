use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

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
        self.mutex.lock().unwrap();
    }

    fn try_lock(&self) -> bool {
        self.mutex.try_lock().is_ok()
    }

    fn unlock(&self) {
        // No-op, as Mutex::unlock is called automatically when the guard is dropped
    }
}

static GLOBAL_MUTEX: PthreadMutex = PthreadMutex::new();
static mut N: i32 = 0;

unsafe extern "C" fn f1(mutex: &PthreadMutex, n: &mut i32) {
    mutex.lock();
    *n += 1;
    mutex.unlock();
    if !mutex.try_lock() {
        return;
    }
    *n += 1;
    mutex.unlock();
}

unsafe extern "C" fn f2(mutex: &PthreadMutex, n: &mut i32) {
    mutex.lock();
    *n += 1;
    mutex.unlock();
    if !mutex.try_lock() {
        return;
    }
    *n += 1;
    mutex.unlock();
}

unsafe extern "C" fn f3(mutex: &PthreadMutex, n: &mut i32) {
    mutex.lock();
    *n += 1;
    mutex.unlock();
    if !mutex.try_lock() {
        return;
    }
    *n += 1;
    mutex.unlock();
    if !mutex.try_lock() {
        return;
    }
    *n += 1;
    mutex.unlock();
}

unsafe extern "C" fn f4(mutex: &PthreadMutex, n: &mut i32) {
    mutex.lock();
    *n += 1;
    mutex.unlock();
    if !mutex.try_lock() {
        return;
    }
    *n += 1;
    mutex.unlock();
    if !mutex.try_lock() {
        return;
    }
    *n += 1;
    mutex.unlock();
}

unsafe extern "C" fn t_fun(mutex: &PthreadMutex, n: &mut i32, _arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(mutex, n);
    f2(mutex, n);
    f3(mutex, n);
    f4(mutex, n);
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mutex = Arc::new(GLOBAL_MUTEX);
    let n = Arc::new(Mutex::new(0));

    let handle1 = {
        let mutex = mutex.clone();
        let n = n.clone();
        thread::spawn(move || {
            t_fun(&mutex, &mut *n.lock().unwrap(), std::ptr::null_mut());
        })
    };

    let handle2 = {
        let mutex = mutex.clone();
        let n = n.clone();
        thread::spawn(move || {
            t_fun(&mutex, &mut *n.lock().unwrap(), std::ptr::null_mut());
        })
    };

    handle1.join().unwrap();
    handle2.join().unwrap();

    let result = *n.lock().unwrap();
    let c_string = CString::new(format!("{}\n", result)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}