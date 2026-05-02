use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct MutexWrapper {
    mutex: Mutex<()>,
}

impl MutexWrapper {
    fn lock(&self) {
        self.mutex.lock().unwrap();
    }

    fn try_lock(&self) -> bool {
        self.mutex.try_lock().is_ok()
    }

    fn unlock(&self) {
        // No-op, as MutexGuard is dropped automatically
    }
}

static N: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
static M: MutexWrapper = MutexWrapper {
    mutex: Mutex::new(()),
};
static R: MutexWrapper = MutexWrapper {
    mutex: Mutex::new(()),
};

unsafe extern "C" fn f1() {
    M.lock();
    let mut n = N.lock().unwrap();
    *n += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    let mut n = N.lock().unwrap();
    *n += 1;
    M.unlock();
}

unsafe extern "C" fn f2() {
    R.lock();
    let mut n = N.lock().unwrap();
    *n += 1;
    R.unlock();
    if !R.try_lock() {
        return;
    }
    let mut n = N.lock().unwrap();
    *n += 1;
    R.unlock();
}

unsafe extern "C" fn f3() {
    M.lock();
    let mut n = N.lock().unwrap();
    *n += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    let mut n = N.lock().unwrap();
    *n += 1;
    M.unlock();
    if !M.try_lock() {
        return;
    }
    let mut n = N.lock().unwrap();
    *n += 1;
    M.unlock();
}

unsafe extern "C" fn f4() {
    R.lock();
    let mut n = N.lock().unwrap();
    *n += 1;
    R.unlock();
    if !R.try_lock() {
        return;
    }
    let mut n = N.lock().unwrap();
    *n += 1;
    R.unlock();
    if !R.try_lock() {
        return;
    }
    let mut n = N.lock().unwrap();
    *n += 1;
    R.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    f4();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let n = N.clone();
    let m = M.clone();
    let r = R.clone();

    let handle1 = thread::spawn(move || {
        f1();
        f2();
        f3();
        f4();
    });

    let handle2 = thread::spawn(move || {
        f1();
        f2();
        f3();
        f4();
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