use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<()>,
}

static SHARED_DATA: Arc<SharedData> = Arc::new(SharedData {
    n1: 0,
    m1: Mutex::new(()),
});

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let data = SHARED_DATA.clone();
    let _guard = data.m1.lock().unwrap();
    data.n1 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let data = SHARED_DATA.clone();
    let _guard = data.m1.lock().unwrap();
    data.n1 += 1;

    let data_clone = data.clone();
    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let data_clone = data.clone();
    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = SHARED_DATA.clone();
    let _guard = data.m1.lock().unwrap();
    data.n1 += 1;

    let c_string = CString::new(format!("{}\n", data.n1)).unwrap();
    libc::printf(c_string.as_ptr());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}