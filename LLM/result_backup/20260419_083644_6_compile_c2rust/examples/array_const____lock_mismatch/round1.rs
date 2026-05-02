use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;

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

static mut N1: [i32; 3] = [0; 3];
static NUM_MUTEX: [PthreadMutex; 3] = [PthreadMutex::new(), PthreadMutex::new(), PthreadMutex::new()];

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
    std::ptr::null_mut()
}

fn main_0() -> c_int {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            unsafe { t_fun(std::ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let output = format!("{} {} {}\n", N1[0], N1[1], N1[2]);
    let c_string = CString::new(output).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}