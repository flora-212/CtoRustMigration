use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::ptr;

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

unsafe extern "C" fn f1() {
    let mut x: i32 = 0;
    NUM_MUTEX.lock();
    while N1 < 10 {
        x = N1;
        NUM_MUTEX.unlock();
        x += 1;
        NUM_MUTEX.lock();
        N1 = x;
    }
    NUM_MUTEX.unlock();
}

unsafe extern "C" fn f2() {
    NUM_MUTEX.lock();
    while N1 < 20 {
        if N1 > 18 {
            NUM_MUTEX.unlock();
            return;
        }
        N1 += 1;
    }
    NUM_MUTEX.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );

    libc::pthread_join(id1, ptr::null_mut());
    libc::pthread_join(id2, ptr::null_mut());

    let c_str = CString::new(format!("{}\n", N1)).unwrap();
    libc::printf(c_str.as_ptr());

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}