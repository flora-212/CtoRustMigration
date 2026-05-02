use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[derive(Debug)]
struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    fn lock(&self) {
        self.mutex.lock().unwrap();
    }

    fn unlock(&self) {
        // No-op in this implementation since Mutex::lock handles unlocking
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    mutex: Mutex::new(()),
};

static mut N1: i32 = 0;

unsafe extern "C" fn f1() {
    NUM_MUTEX.lock();
    NUM_MUTEX.lock();
    N1 += 1;
    NUM_MUTEX.unlock();
    NUM_MUTEX.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    N1 += 1;

    let handle1 = thread::spawn(|| {
        f1();
    });

    let handle2 = thread::spawn(|| {
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    N1 += 1;
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}