use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use std::ptr;

#[derive(Debug)]
pub struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    pub fn lock(&self) {
        self.mutex.lock().unwrap();
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    mutex: Mutex::new(()),
};

static mut N1: i32 = 0;

unsafe extern "C" fn inc() {
    N1 += 1;
}

unsafe extern "C" fn f1() {
    NUM_MUTEX.lock();
    inc();
}

unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let num_mutex = Arc::new(NUM_MUTEX);
    let n1 = Arc::new(Mutex::new(0));

    let num_mutex_clone1 = Arc::clone(&num_mutex);
    let n1_clone1 = Arc::clone(&n1);
    let handle1 = thread::spawn(move || {
        num_mutex_clone1.lock();
        *n1_clone1.lock().unwrap() += 1;
    });

    let num_mutex_clone2 = Arc::clone(&num_mutex);
    let n1_clone2 = Arc::clone(&n1);
    let handle2 = thread::spawn(move || {
        num_mutex_clone2.lock();
        *n1_clone2.lock().unwrap() += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", *n1.lock().unwrap());

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}