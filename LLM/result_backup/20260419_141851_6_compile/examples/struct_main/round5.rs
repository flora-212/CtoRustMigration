use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<i32>,
}

lazy_static::lazy_static! {
    static ref SHARED_DATA: Arc<Mutex<SharedData>> = Arc::new(Mutex::new(SharedData {
        n1: 0,
        m1: Mutex::new(0),
    }));
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let data = SHARED_DATA.lock().unwrap();
    let mut inner_data = data.m1.lock().unwrap();
    *inner_data += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let data = SHARED_DATA.lock().unwrap();

    let mut handles = vec![];

    for _ in 0..2 {
        let data_clone = SHARED_DATA.clone();
        let handle = thread::spawn(move || {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut inner_data = data.m1.lock().unwrap();
    *inner_data += 1;

    let c_string = CString::new(format!("{}\n", *inner_data)).unwrap();
    libc::printf(c_string.as_ptr());
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}