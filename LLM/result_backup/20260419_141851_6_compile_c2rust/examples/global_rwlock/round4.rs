use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

#[no_mangle]
pub static N: AtomicI32 = AtomicI32::new(0);

#[no_mangle]
pub static LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));

#[no_mangle]
pub extern "C" fn f1() -> i32 {
    let lock = LOCK.lock().unwrap();
    let x = N.load(Ordering::SeqCst);
    drop(lock);
    x
}

#[no_mangle]
pub extern "C" fn f2() {
    let lock = LOCK.lock().unwrap();
    N.fetch_add(1, Ordering::SeqCst);
    drop(lock);
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut i8) -> *mut i8 {
    f1();
    f2();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let lock1 = LOCK.clone();
    let lock2 = LOCK.clone();

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = N.load(Ordering::SeqCst);
    println!("{}", n);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}