use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

#[no_mangle]
pub extern "C" fn f1(shared_data: &Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    *num_mutex += 1;
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let shared_data = unsafe { Arc::from_raw(arg as *const SharedData) };
    f1(&shared_data);
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(0),
    });

    let shared_data_clone = Arc::clone(&shared_data);
    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_data_clone) as *mut std::ffi::c_void);
    });

    let shared_data_clone = Arc::clone(&shared_data);
    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_data_clone) as *mut std::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    *num_mutex += 1;

    println!("{}", *num_mutex);
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
