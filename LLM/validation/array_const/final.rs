use std::sync::{Arc, Mutex};
use std::thread;

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

    fn lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.mutex.lock().unwrap()
    }
}

lazy_static::lazy_static! {
    static ref NUM_MUTEX: [PthreadMutex; 2] = [
        PthreadMutex::new(),
        PthreadMutex::new()
    ];
    static ref N1: Arc<Mutex<[i32; 2]>> = Arc::new(Mutex::new([0; 2]));
}

// 线程1：顺序 0 -> 1
unsafe extern "C" fn f1() {
    let _l0 = NUM_MUTEX[0].lock();
    let _l1 = NUM_MUTEX[1].lock();

    let mut n1 = N1.lock().unwrap();
    n1[0] += 1;
}

// 线程2：顺序 1 -> 0 ❗
unsafe extern "C" fn f2() {
    let _l1 = NUM_MUTEX[1].lock();
    let _l0 = NUM_MUTEX[0].lock();

    let mut n1 = N1.lock().unwrap();
    n1[1] += 1;
}

unsafe extern "C" fn t1(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe extern "C" fn t2(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f2();
    std::ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        unsafe { t1(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t2(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}