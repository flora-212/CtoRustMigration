use std::sync::{Arc, Mutex};
use std::thread;

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
        let _guard = self.mutex.lock().unwrap();
    }

    pub fn try_lock(&self) -> bool {
        self.mutex.try_lock().is_ok()
    }
}

static mut N: i32 = 0;
static M: PthreadMutex = PthreadMutex::new();

#[no_mangle]
pub extern "C" fn f1() {
    M.lock();
    unsafe { N += 1; }
    M.lock();
    unsafe { N += 1; }
}

#[no_mangle]
pub extern "C" fn f2() {
    M.lock();
    unsafe { N += 1; }
    M.lock();
    unsafe { N += 1; }
}

#[no_mangle]
pub extern "C" fn f3() {
    M.lock();
    unsafe { N += 1; }
    M.lock();
    unsafe { N += 1; }
    M.lock();
    unsafe { N += 1; }
}

#[no_mangle]
pub extern "C" fn f4() {
    M.lock();
    unsafe { N += 1; }
    M.lock();
    unsafe { N += 1; }
    M.lock();
    unsafe { N += 1; }
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    f4();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let m = Arc::new(Mutex::new(()));
    let m_clone = Arc::clone(&m);

    let handle1 = thread::spawn(move || {
        let _guard = m.lock().unwrap();
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        let _guard = m_clone.lock().unwrap();
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe { libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N); }
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}