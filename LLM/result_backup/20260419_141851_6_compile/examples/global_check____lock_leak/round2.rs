use std::sync::{Arc, Mutex};
use std::thread;
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
        let _lock = self.mutex.lock().unwrap();
    }

    fn try_lock(&self) -> bool {
        self.mutex.try_lock().is_ok()
    }

    fn unlock(&self) {
        // No-op in this implementation since Mutex::lock and try_lock handle unlocking
    }
}

static N: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
static M: PthreadMutex = PthreadMutex::new();

unsafe extern "C" fn f1(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
}

unsafe extern "C" fn f2(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
}

unsafe extern "C" fn f3(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
}

unsafe extern "C" fn f4(n: &Arc<Mutex<i32>>) {
    M.lock();
    *n.lock().unwrap() += 1;
    if !M.try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.lock();
    *n.lock().unwrap() += 1;
    M.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let n = Arc::clone(&N);
    f1(&n);
    f2(&n);
    f3(&n);
    f4(&n);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let n1 = Arc::clone(&N);
    let n2 = Arc::clone(&N);

    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = *N.lock().unwrap();
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}