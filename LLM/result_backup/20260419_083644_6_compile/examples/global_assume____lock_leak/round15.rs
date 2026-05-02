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
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    mutex: Mutex::new(()),
};

static mut N1: i32 = 0;

unsafe fn inc() {
    N1 += 1;
}

unsafe fn f1() {
    NUM_MUTEX.lock();
    inc();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let num_mutex = Arc::new(NUM_MUTEX);
    let mut handles = vec![];

    for _ in 0..2 {
        let num_mutex_clone = Arc::clone(&num_mutex);
        let handle = thread::spawn(move || {
            unsafe {
                t_fun(std::ptr::null_mut());
            }
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
    unsafe {
        std::process::exit(main_0() as i32);
    }
}