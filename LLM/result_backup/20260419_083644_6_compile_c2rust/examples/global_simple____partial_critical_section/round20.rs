use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    num_mutex: Mutex<()>,
}

fn f1(shared_data: Arc<Mutex<SharedData>>) {
    let mut x = {
        let data = shared_data.lock().unwrap();
        data.n3
    };

    let mut data = shared_data.lock().unwrap();
    data.n1 += x;
    data.n2 += x;
    data.n3 += x;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared_data = Arc::from_raw(arg as *const Mutex<SharedData>);
    f1(shared_data);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let shared_data = Arc::new(Mutex::new(SharedData {
        n1: 0,
        n2: 0,
        n3: 1,
        num_mutex: Mutex::new(()),
    }));

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || {
        let raw_ptr = Arc::into_raw(shared_data_clone1);
        unsafe { t_fun(raw_ptr as *mut libc::c_void) };
    });

    let handle2 = thread::spawn(move || {
        let raw_ptr = Arc::into_raw(shared_data_clone2);
        unsafe { t_fun(raw_ptr as *mut libc::c_void) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = shared_data.lock().unwrap();
    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr(),
            data.n1,
            data.n2,
            data.n3,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}