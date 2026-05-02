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
        self.mutex.lock().unwrap();
    }

    pub fn try_lock(&self) -> bool {
        self.mutex.try_lock().is_ok()
    }

    pub fn unlock(&self) {
        // No-op in this implementation since Mutex::unlock is called automatically
    }
}

static mut N: i32 = 0;
static M: PthreadMutex = PthreadMutex::new();

unsafe extern "C" fn f1() {
    M.lock();
    N += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    N += 1;
    M.unlock();
}

unsafe extern "C" fn f2() {
    M.lock();
    N += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    N += 1;
    M.unlock();
}

unsafe extern "C" fn f3() {
    M.lock();
    N += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    N += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    N += 1;
    M.unlock();
}

unsafe extern "C" fn f4() {
    M.lock();
    N += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    N += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    N += 1;
    M.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    f4();
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );

    libc::pthread_join(id1, std::ptr::null_mut());
    libc::pthread_join(id2, std::ptr::null_mut());

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}