use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::c_void;

static INIT: Once = Once::new();
static mut NUM_MUTEX: Option<Arc<Mutex<i32>>> = None;

#[no_mangle]
pub static mut n1: i32 = 0;

fn init() {
    unsafe {
        NUM_MUTEX = Some(Arc::new(Mutex::new(0)));
    }
}

#[no_mangle]
pub unsafe extern "C" fn inc() {
    INIT.call_once(init);
    let num_mutex = NUM_MUTEX.as_ref().unwrap();
    let mut n1_lock = num_mutex.lock().unwrap();
    *n1_lock += 1;
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    inc();
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    INIT.call_once(init);

    let num_mutex_clone = NUM_MUTEX.as_ref().unwrap().clone();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1);
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
