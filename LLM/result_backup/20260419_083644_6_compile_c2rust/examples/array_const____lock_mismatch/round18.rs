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

static NUM_MUTEX: [PthreadMutex; 3] = [PthreadMutex::new(), PthreadMutex::new(), PthreadMutex::new()];

static N1: Arc<Mutex<[i32; 3]>> = Arc::new(Mutex::new([0; 3]));

unsafe extern "C" fn f1(n1: &Arc<Mutex<[i32; 3]>>) {
    n1.lock().unwrap()[0] += 1;
    n1.lock().unwrap()[1] += 1;
    n1.lock().unwrap()[0] += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let n1 = Arc::from_raw(arg as *const Arc<Mutex<[i32; 3]>>);
    f1(&n1);
    std::ptr::null_mut()
}

fn main_0() -> c_int {
    let n1 = Arc::new(Mutex::new([0; 3]));
    let mut handles = vec![];

    for _ in 0..2 {
        let n1_clone = Arc::clone(&n1);
        let handle = thread::spawn(move || {
            unsafe { t_fun(Arc::into_raw(n1_clone) as *mut libc::c_void) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let output = format!("{} {} {}\n", n1.lock().unwrap()[0], n1.lock().unwrap()[1], n1.lock().unwrap()[2]);
    let c_string = CString::new(output).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}