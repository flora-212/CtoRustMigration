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

    fn unlock(&self) {
        // No-op in Rust, as Mutex automatically releases the lock when dropped
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex::new();
static mut N1: i32 = 0;

unsafe extern "C" fn f1(mutex: &PthreadMutex) {
    let mut x: i32 = 0;
    mutex.lock();
    while N1 < 10 {
        x = N1;
        mutex.unlock();
        x += 1;
        mutex.lock();
        N1 = x;
    }
    mutex.unlock();
}

unsafe extern "C" fn f2(mutex: &PthreadMutex) {
    mutex.lock();
    while N1 < 20 {
        if N1 > 18 {
            mutex.unlock();
            return;
        }
        N1 += 1;
    }
    mutex.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(&NUM_MUTEX);
    f2(&NUM_MUTEX);
    std::ptr::null_mut()
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

    let c_str = CString::new(format!("{}\n", N1)).unwrap();
    libc::printf(c_str.as_ptr());

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}