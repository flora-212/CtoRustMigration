use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;

#[derive(Debug)]
struct RwLock {
    lock: Mutex<()>,
    readers: Mutex<usize>,
}

impl RwLock {
    const fn new() -> Self {
        RwLock {
            lock: Mutex::new(()),
            readers: Mutex::new(0),
        }
    }

    fn read(&self) {
        let mut readers = self.readers.lock().unwrap();
        *readers += 1;
        if *readers == 1 {
            self.lock.lock().unwrap();
        }
    }

    fn read_unlock(&self) {
        let mut readers = self.readers.lock().unwrap();
        *readers -= 1;
        if *readers == 0 {
            drop(self.lock.lock().unwrap());
        }
    }

    fn write(&self) {
        self.lock.lock().unwrap();
    }

    fn write_unlock(&self) {
        drop(self.lock.lock().unwrap());
    }
}

static LOCK: RwLock = RwLock::new();
static mut N: i32 = 0;

#[no_mangle]
pub extern "C" fn f1() -> i32 {
    LOCK.read();
    let x = unsafe { N };
    LOCK.read_unlock();
    x
}

#[no_mangle]
pub extern "C" fn f2() {
    LOCK.write();
    unsafe {
        N += 1;
    }
    LOCK.write_unlock();
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    std::ptr::null_mut()
}

static LOCK_ARC: Arc<RwLock> = Arc::new(RwLock::new());

unsafe fn main_0() -> i32 {
    let mut handles = vec![];

    for _ in 0..2 {
        let lock_clone = LOCK_ARC.clone();
        let handle = thread::spawn(move || {
            t_fun(std::ptr::null_mut());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}