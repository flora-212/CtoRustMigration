use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::CString;

static INIT: Once = Once::new();
static SHARED_DATA: Arc<SharedData> = Arc::new(SharedData {
    n1: Mutex::new(0),
    n2: Mutex::new(0),
});

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let shared_data = SHARED_DATA.clone();
    let mut n1 = shared_data.n1.lock().unwrap();
    *n1 += 1;
    let mut n2 = shared_data.n2.lock().unwrap();
    *n2 += 1;
    *n1 += 1;
    *n1 += *n2;
    *n1 += 1;
    *n2 += 1;
    *n2 += *n1;
    *n2 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(|| {
        // Reinitialize SHARED_DATA if needed
    });

    let shared_data = SHARED_DATA.clone();
    let handle1 = thread::spawn(move || {
        f1();
    });
    let handle2 = thread::spawn(move || {
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared_data.n1.lock().unwrap();
    let n2 = shared_data.n2.lock().unwrap();
    let c_string = CString::new(format!("{} {}\n", *n1, *n2)).unwrap();
    libc::printf(c_string.as_ptr());
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}