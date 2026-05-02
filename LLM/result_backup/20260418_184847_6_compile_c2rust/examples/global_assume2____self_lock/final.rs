use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;
use std::ffi::CString;

#[derive(Debug)]
struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    fn lock(&self) {
        self.mutex.lock().unwrap();
    }

    fn unlock(&self) {
        // No-op in this case since we don't need to unlock explicitly
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    mutex: Mutex::new(()),
};

lazy_static::lazy_static! {
    static ref N1: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

unsafe extern "C" fn inc(n1: &Arc<Mutex<i32>>) -> i32 {
    let mut n = n1.lock().unwrap();
    *n += 1;
    if *n != 0 {
        *n
    } else {
        *n + 1
    }
}

unsafe extern "C" fn f1(n1: &Arc<Mutex<i32>>) {
    NUM_MUTEX.lock();
    NUM_MUTEX.lock();
    inc(n1);
    NUM_MUTEX.unlock();
    NUM_MUTEX.unlock();
}

fn t_fun(n1: Arc<Mutex<i32>>) {
    unsafe {
        f1(&n1);
    }
}

fn main_0() -> i32 {
    let n1 = Arc::clone(&N1);
    let handle1 = thread::spawn(move || t_fun(n1));

    let n2 = Arc::clone(&N1);
    let handle2 = thread::spawn(move || t_fun(n2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = N1.lock().unwrap();
    println!("{}", *n);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
