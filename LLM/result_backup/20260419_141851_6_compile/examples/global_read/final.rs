use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    num_mutex: Mutex<()>,
}

#[no_mangle]
pub fn f1(shared_data: Arc<Mutex<SharedData>>) {
    let mut data = shared_data.lock().unwrap();
    data.n1 += data.n2;
}

#[no_mangle]
pub fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let shared_data = unsafe { Arc::from_raw(arg as *const Mutex<SharedData>) };
    f1(shared_data);
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let shared_data = Arc::new(Mutex::new(SharedData {
        n1: 0,
        n2: 0,
        num_mutex: Mutex::new(()),
    }));

    let mut handles = vec![];

    for _ in 0..2 {
        let cloned_data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            t_fun(Arc::into_raw(cloned_data) as *mut std::ffi::c_void);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = shared_data.lock().unwrap();
    println!("{} {}", data.n1, data.n2);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
