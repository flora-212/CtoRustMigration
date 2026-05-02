use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    num_mutex: Mutex<()>,
    cond: Condvar,
}

fn f1(shared_data: Arc<Mutex<SharedData>>) {
    let mut n1 = shared_data.lock().unwrap();
    n1.n2 += 1;
    if n1.n1 == 0 {
        n1.n1 += 1;
        n1 = shared_data.lock().unwrap();
    } else {
        shared_data.lock().unwrap().cond.notify_one();
    }
    n1.n2 += 1;
    if n1.n2 == 1 {
        n1 = shared_data.lock().unwrap();
    } else {
        shared_data.lock().unwrap().cond.notify_all();
    }
    n1.n1 += 1;
}

fn t_fun(shared_data: Arc<Mutex<SharedData>>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(Mutex::new(SharedData {
        n1: 0,
        n2: 0,
        num_mutex: Mutex::new(()),
        cond: Condvar::new(),
    }));

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || t_fun(shared_data_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared_data.lock().unwrap().n1;
    let n2 = shared_data.lock().unwrap().n2;

    unsafe {
        libc::printf(b"%d %d\n\0".as_ptr() as *const libc::c_char, n1, n2);
    }

    0
}

fn main() {
    std::process::exit(main_0());
}
