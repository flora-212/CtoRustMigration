use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
}

fn f1(shared_data: Arc<Mutex<SharedData>>) {
    let mut data = shared_data.lock().unwrap();
    let x = data.n3;
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
    let shared_data = Arc::new(Mutex::new(SharedData { n1: 0, n2: 0, n3: 1 }));

    let mut handles = vec![];

    for _ in 0..2 {
        let shared_data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            unsafe {
                t_fun(Arc::into_raw(shared_data_clone) as *mut libc::c_void);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

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