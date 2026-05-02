use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::c_void;

static INIT: Once = Once::new();
static mut NUM_MUTEX: Option<Arc<Mutex<i32>>> = None;

#[no_mangle]
pub static mut N1: i32 = 0;

fn init() {
    unsafe {
        NUM_MUTEX = Some(Arc::new(Mutex::new(0)));
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut num_mutex = NUM_MUTEX.as_ref().unwrap().lock().unwrap();
    *num_mutex += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    INIT.call_once(init);

    N1 += 1;

    let num_mutex_clone = NUM_MUTEX.as_ref().unwrap().clone();
    let handle1 = thread::spawn(move || {
        let mut num_mutex = num_mutex_clone.lock().unwrap();
        *num_mutex += 1;
    });

    let num_mutex_clone = NUM_MUTEX.as_ref().unwrap().clone();
    let handle2 = thread::spawn(move || {
        let mut num_mutex = num_mutex_clone.lock().unwrap();
        *num_mutex += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    N1 += 1;

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
