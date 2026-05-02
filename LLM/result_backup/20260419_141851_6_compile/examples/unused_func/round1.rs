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
        // No-op in this case, as Mutex automatically unlocks when dropped
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    mutex: Mutex::new(()),
};

static mut N1: i32 = 0;

unsafe extern "C" fn f1() {
    NUM_MUTEX.lock();
    N1 += 1;
    NUM_MUTEX.unlock();
}

unsafe extern "C" fn f2() {
    NUM_MUTEX.lock();
    N1 += 1;
    NUM_MUTEX.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}