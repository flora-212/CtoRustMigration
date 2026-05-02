use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    fn lock(&self) {
        self.mutex.lock().unwrap();
    }

    fn unlock(&self) {
        // No-op in this implementation since Mutex automatically unlocks
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

fn main_0() -> i32 {
    unsafe {
        N1 += 1;
    }

    let handle1 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        N1 += 1;
    }

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    }

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}
