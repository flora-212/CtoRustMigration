use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

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

static mut N: Option<Arc<Mutex<i32>>> = None;
static mut M: Option<PthreadMutex> = None;
static INIT: Once = Once::new();

fn initialize_globals() {
    unsafe {
        N = Some(Arc::new(Mutex::new(0)));
        M = Some(PthreadMutex::new());
    }
}

unsafe extern "C" fn f1(n: &Arc<Mutex<i32>>) {
    M.as_ref().unwrap().lock();
    *n.lock().unwrap() += 1;
    M.as_ref().unwrap().unlock();
    if !M.as_ref().unwrap().try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.as_ref().unwrap().unlock();
}

unsafe extern "C" fn f2(n: &Arc<Mutex<i32>>) {
    M.as_ref().unwrap().lock();
    *n.lock().unwrap() += 1;
    M.as_ref().unwrap().unlock();
    if !M.as_ref().unwrap().try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
}

unsafe extern "C" fn f3(n: &Arc<Mutex<i32>>) {
    M.as_ref().unwrap().lock();
    *n.lock().unwrap() += 1;
    if !M.as_ref().unwrap().try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.as_ref().unwrap().unlock();
    if !M.as_ref().unwrap().try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
}

unsafe extern "C" fn f4(n: &Arc<Mutex<i32>>) {
    M.as_ref().unwrap().lock();
    *n.lock().unwrap() += 1;
    if !M.as_ref().unwrap().try_lock() {
        return;
    }
    *n.lock().unwrap() += 1;
    M.as_ref().unwrap().lock();
    *n.lock().unwrap() += 1;
    M.as_ref().unwrap().unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let n = Arc::clone(N.as_ref().unwrap());
    f1(&n);
    f2(&n);
    f3(&n);
    f4(&n);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(|| {
        initialize_globals();
    });

    let n1 = Arc::clone(N.as_ref().unwrap());
    let n2 = Arc::clone(N.as_ref().unwrap());

    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = *N.as_ref().unwrap().lock().unwrap();
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
