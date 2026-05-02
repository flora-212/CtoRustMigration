use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    fn lock(&self) {
        self.mutex.lock().unwrap();
    }

    fn unlock(&self) {
        // No-op in Rust's Mutex, as unlock is implicit when the guard is dropped
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    mutex: Mutex::new(()),
};

static mut N1: i32 = 0;

#[no_mangle]
pub extern "C" fn lock() {
    NUM_MUTEX.lock();
}

#[no_mangle]
pub extern "C" fn unlock() {
    NUM_MUTEX.unlock();
}

#[no_mangle]
pub extern "C" fn f1() {
    lock();
    unsafe {
        N1 += 1;
    }
    unlock();
}

#[no_mangle]
pub extern "C" fn lock2(n: i32) -> i32 {
    NUM_MUTEX.lock();
    unsafe {
        N1 += n;
    }
    N1
}

#[no_mangle]
pub extern "C" fn unlock2(n: i32) -> i32 {
    unsafe {
        N1 += n;
    }
    let n2 = N1;
    NUM_MUTEX.unlock();
    n2
}

#[no_mangle]
pub extern "C" fn f2() -> i32 {
    let n2 = lock2(1);
    unsafe {
        N1 += 1;
    }
    let n2 = n2 + unlock2(1);
    n2
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}