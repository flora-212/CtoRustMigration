use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;

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

unsafe extern "C" fn f1() {
    GLOBAL_MUTEX.lock();
    N += 1;
    GLOBAL_MUTEX.unlock();
    if !GLOBAL_MUTEX.try_lock() {
        return;
    }
    N += 1;
    GLOBAL_MUTEX.unlock();
}

unsafe extern "C" fn f2() {
    GLOBAL_MUTEX.lock();
    N += 1;
    GLOBAL_MUTEX.unlock();
    if !GLOBAL_MUTEX.try_lock() {
        return;
    }
    N += 1;
    GLOBAL_MUTEX.unlock();
}

unsafe extern "C" fn f3() {
    GLOBAL_MUTEX.lock();
    N += 1;
    GLOBAL_MUTEX.unlock();
    if !GLOBAL_MUTEX.try_lock() {
        return;
    }
    N += 1;
    GLOBAL_MUTEX.unlock();
    if !GLOBAL_MUTEX.try_lock() {
        return;
    }
    N += 1;
    GLOBAL_MUTEX.unlock();
}

unsafe extern "C" fn f4() {
    GLOBAL_MUTEX.lock();
    N += 1;
    GLOBAL_MUTEX.unlock();
    if !GLOBAL_MUTEX.try_lock() {
        return;
    }
    N += 1;
    GLOBAL_MUTEX.unlock();
    if !GLOBAL_MUTEX.try_lock() {
        return;
    }
    N += 1;
    GLOBAL_MUTEX.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    f4();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });
    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N);
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}