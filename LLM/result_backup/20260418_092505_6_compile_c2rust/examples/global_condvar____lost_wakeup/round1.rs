use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    num_mutex: Mutex<()>,
    cond: Condvar,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    shared_data.n1 += 1;
    if shared_data.n1 == 1 {
        num_mutex = shared_data.cond.wait(num_mutex).unwrap();
    } else {
        num_mutex = shared_data.cond.wait(num_mutex).unwrap();
    }

    num_mutex = shared_data.num_mutex.lock().unwrap();
    shared_data.n2 += 1;
    if shared_data.n2 == 1 {
        num_mutex = shared_data.cond.wait(num_mutex).unwrap();
    } else {
        shared_data.cond.notify_all();
    }
}

fn main() {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        num_mutex: Mutex::new(()),
        cond: Condvar::new(),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || {
        f1(shared_data_clone1);
    });

    let handle2 = thread::spawn(move || {
        f1(shared_data_clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {}", shared_data.n1, shared_data.n2);
}