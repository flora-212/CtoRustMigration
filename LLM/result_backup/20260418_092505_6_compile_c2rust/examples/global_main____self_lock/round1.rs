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

    N1 += 1;
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}