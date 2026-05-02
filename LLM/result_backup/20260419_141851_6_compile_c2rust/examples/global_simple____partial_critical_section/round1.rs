use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    mutex: Mutex<()>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut x = 0;
    {
        let mut lock = shared_data.mutex.lock().unwrap();
        x = shared_data.n3;
        shared_data.n1 += x;
        shared_data.n2 += x;
        shared_data.n3 += x;
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        n3: 1,
        mutex: Mutex::new(()),
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
            shared_data.n1,
            shared_data.n2,
            shared_data.n3,
        );
    }

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}