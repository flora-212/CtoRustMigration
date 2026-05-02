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
}

static N: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
static M: PthreadMutex = PthreadMutex::new();
static R: PthreadMutex = PthreadMutex::new();

unsafe extern "C" fn f1() {
    M.lock();
    *N.lock().unwrap() += 1;
    M.lock();
    *N.lock().unwrap() += 1;
}

unsafe extern "C" fn f2() {
    R.lock();
    *N.lock().unwrap() += 1;
    if !R.try_lock() {
        return;
    }
    *N.lock().unwrap() += 1;
}

unsafe extern "C" fn f3() {
    M.lock();
    *N.lock().unwrap() += 1;
    if !M.lock() {
        return;
    }
    *N.lock().unwrap() += 1;
    if !M.try_lock() {
        return;
    }
    *N.lock().unwrap() += 1;
}

unsafe extern "C" fn f4() {
    R.lock();
    *N.lock().unwrap() += 1;
    if !R.try_lock() {
        return;
    }
    *N.lock().unwrap() += 1;
    if !R.lock() {
        return;
    }
    *N.lock().unwrap() += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    f4();
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

    let n_value = *N.lock().unwrap();
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n_value);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}