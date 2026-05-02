use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;

#[derive(Debug)]
struct MutexWrapper {
    mutex: Mutex<i32>,
}

impl MutexWrapper {
    fn new(value: i32) -> Self {
        MutexWrapper {
            mutex: Mutex::new(value),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, i32> {
        self.mutex.lock().unwrap()
    }
}

static NUM_MUTEX1: MutexWrapper = MutexWrapper::new(0);
static NUM_MUTEX2: MutexWrapper = MutexWrapper::new(0);

unsafe extern "C" fn f1() {
    let mut num1 = NUM_MUTEX1.lock();
    *num1 += 1;
}

unsafe extern "C" fn f2() {
    let mut num2 = NUM_MUTEX2.lock();
    *num2 += 1;
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

    let num1 = NUM_MUTEX1.lock();
    let c_string = CString::new(format!("{}\n", *num1)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}