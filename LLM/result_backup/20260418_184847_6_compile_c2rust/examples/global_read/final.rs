use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    num_mutex: Mutex<()>,
}

fn f1(shared_data: Arc<Mutex<SharedData>>) {
    let mut data = shared_data.lock().unwrap();
    data.n1 += data.n2;
}

fn t_fun(shared_data: Arc<Mutex<SharedData>>) {
    f1(shared_data);
}

fn main_0() -> libc::c_int {
    let shared_data = Arc::new(Mutex::new(SharedData {
        n1: 0,
        n2: 0,
        num_mutex: Mutex::new(()),
    }));

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || {
        t_fun(shared_data_clone1);
    });

    let handle2 = thread::spawn(move || {
        t_fun(shared_data_clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = shared_data.lock().unwrap();
    unsafe {
        libc::printf(
            CString::new("%d %d\n").unwrap().as_ptr(),
            data.n1,
            data.n2,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
