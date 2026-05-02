use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    n3: Mutex<i32>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut x = 0;
    {
        let mut lock = shared_data.n3.lock().unwrap();
        x = *lock;
        *lock += x;
    }
    {
        let mut lock = shared_data.n1.lock().unwrap();
        *lock += x;
    }
    {
        let mut lock = shared_data.n2.lock().unwrap();
        *lock += x;
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
        n2: Mutex::new(0),
        n3: Mutex::new(1),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let shared_data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            t_fun(shared_data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            *shared_data.n1.lock().unwrap(),
            *shared_data.n2.lock().unwrap(),
            *shared_data.n3.lock().unwrap(),
        );
    }

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}
