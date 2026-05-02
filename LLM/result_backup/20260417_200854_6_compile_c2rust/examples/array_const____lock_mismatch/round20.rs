use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
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
        let _lock = self.mutex.lock().unwrap();
    }

    fn unlock(&self) {
        // No-op in Rust, as MutexGuard is dropped automatically
    }
}

static NUM_MUTEX: [PthreadMutex; 3] = [
    PthreadMutex::new(),
    PthreadMutex::new(),
    PthreadMutex::new(),
];

static mut N1: [i32; 3] = [0; 3];

unsafe extern "C" fn f1() {
    NUM_MUTEX[0].lock();
    N1[0] += 1;
    NUM_MUTEX[0].unlock();

    NUM_MUTEX[1].lock();
    N1[1] += 1;
    NUM_MUTEX[1].unlock();

    NUM_MUTEX[2].lock();
    N1[0] += 1;
    NUM_MUTEX[2].unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
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

    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        N1[0],
        N1[1],
        N1[2],
    );

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}