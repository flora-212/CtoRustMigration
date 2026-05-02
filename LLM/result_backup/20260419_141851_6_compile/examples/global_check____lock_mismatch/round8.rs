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
        let _guard = self.mutex.lock().unwrap();
    }

    fn try_lock(&self) -> bool {
        self.mutex.try_lock().is_ok()
    }

    fn unlock(&self) {
        // No-op, as the guard will automatically release the lock when dropped
    }
}

static N: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
static M: PthreadMutex = PthreadMutex::new();
static R: PthreadMutex = PthreadMutex::new();

unsafe extern "C" fn f1(n: &Arc<Mutex<i32>>, m: &PthreadMutex) {
    m.lock();
    let mut n = n.lock().unwrap();
    *n += 1;
    m.unlock();
    if !m.try_lock() {
        return;
    }
    let mut n = n.lock().unwrap();
    *n += 1;
    m.unlock();
}

unsafe extern "C" fn f2(n: &Arc<Mutex<i32>>, r: &PthreadMutex) {
    r.lock();
    let mut n = n.lock().unwrap();
    *n += 1;
    r.unlock();
    if !r.try_lock() {
        return;
    }
    let mut n = n.lock().unwrap();
    *n += 1;
    r.unlock();
}

unsafe extern "C" fn f3(n: &Arc<Mutex<i32>>, m: &PthreadMutex) {
    m.lock();
    let mut n = n.lock().unwrap();
    *n += 1;
    m.unlock();
    if !m.try_lock() {
        return;
    }
    let mut n = n.lock().unwrap();
    *n += 1;
    m.unlock();
    if !m.try_lock() {
        return;
    }
    let mut n = n.lock().unwrap();
    *n += 1;
    m.unlock();
}

unsafe extern "C" fn f4(n: &Arc<Mutex<i32>>, r: &PthreadMutex) {
    r.lock();
    let mut n = n.lock().unwrap();
    *n += 1;
    r.unlock();
    if !r.try_lock() {
        return;
    }
    let mut n = n.lock().unwrap();
    *n += 1;
    r.unlock();
    if !r.try_lock() {
        return;
    }
    let mut n = n.lock().unwrap();
    *n += 1;
    r.unlock();
}

unsafe extern "C" fn t_fun(n: &Arc<Mutex<i32>>, m: &PthreadMutex, r: &PthreadMutex) -> *mut libc::c_void {
    f1(n, m);
    f2(n, r);
    f3(n, m);
    f4(n, r);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let n = N.clone();
    let m = &M;
    let r = &R;

    let handle1 = thread::spawn(move || {
        t_fun(&n, m, r);
    });

    let handle2 = thread::spawn(move || {
        t_fun(&n, m, r);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = N.lock().unwrap();
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *n);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}