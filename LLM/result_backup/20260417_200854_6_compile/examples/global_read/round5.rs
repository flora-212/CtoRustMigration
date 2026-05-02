use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    num_mutex: Mutex<()>,
}

#[no_mangle]
pub fn f1(shared_data: Arc<SharedData>) {
    let mut data = shared_data.lock().unwrap();
    data.n1 += data.n2;
}

#[no_mangle]
pub fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let shared_data = unsafe { Arc::from_raw(arg as *const SharedData) };
    f1(shared_data);
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        num_mutex: Mutex::new(()),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_data_clone1) as *mut std::ffi::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_data_clone2) as *mut std::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = shared_data.lock().unwrap();
    println!("{} {}", data.n1, data.n2);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}