use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    num_mutex: Mutex<()>,
    cond: Condvar,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    let mut n1 = shared_data.n1.lock().unwrap();
    *n1 += 1;
    if *n1 == 1 {
        num_mutex = shared_data.cond.wait(num_mutex).unwrap();
    } else {
        shared_data.cond.notify_one();
    }

    let mut n2 = shared_data.n2.lock().unwrap();
    *n2 += 1;
    if *n2 == 1 {
        num_mutex = shared_data.cond.wait(num_mutex).unwrap();
    } else {
        shared_data.cond.notify_all();
    }
}

fn main() {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
        n2: Mutex::new(0),
        num_mutex: Mutex::new(()),
        cond: Condvar::new(),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let handle1 = thread::spawn(move || {
        f1(shared_data_clone1);
    });

    let shared_data_clone2 = Arc::clone(&shared_data);
    let handle2 = thread::spawn(move || {
        f1(shared_data_clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared_data.n1.lock().unwrap();
    let n2 = shared_data.n2.lock().unwrap();
    println!("{} {}", *n1, *n2);
}
