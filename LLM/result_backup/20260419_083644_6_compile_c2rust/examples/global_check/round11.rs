use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
pub struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    pub fn new() -> Self {
        PthreadMutex {
            mutex: Mutex::new(()),
        }
    }

    pub fn lock(&self) {
        self.mutex.lock().unwrap();
    }

    pub fn try_lock(&self) -> bool {
        self.mutex.try_lock().is_ok()
    }

    pub fn unlock(&self) {
        // No-op in this implementation since Mutex::unlock is called automatically
    }
}

lazy_static::lazy_static! {
    static ref N: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    static ref M: PthreadMutex = PthreadMutex::new();
}

unsafe extern "C" fn f1(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
}

unsafe extern "C" fn f2(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
}

unsafe extern "C" fn f3(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
}

unsafe extern "C" fn f4(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let n = N.clone();
    f1(&n);
    f2(&n);
    f3(&n);
    f4(&n);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let n1 = N.clone();
    let n2 = N.clone();

    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = N.lock().unwrap();
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *n);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}