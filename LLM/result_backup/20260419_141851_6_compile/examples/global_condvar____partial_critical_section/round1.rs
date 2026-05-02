use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    num_mutex: Mutex<i32>,
    cond: Condvar,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut n2 = shared_data.n2.lock().unwrap();
    *n2 += 1;

    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    *num_mutex += 1;

    if *num_mutex == 1 {
        num_mutex = shared_data.cond.wait(num_mutex).unwrap();
    } else {
        shared_data.cond.notify_one();
    }

    *n2 += 1;

    if *n2 == 1 {
        num_mutex = shared_data.cond.wait(num_mutex).unwrap();
    } else {
        shared_data.cond.notify_all();
    }

    *num_mutex += 1;
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        num_mutex: Mutex::new(0),
        cond: Condvar::new(),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let handle1 = thread::spawn(move || t_fun(shared_data_clone1));

    let shared_data_clone2 = Arc::clone(&shared_data);
    let handle2 = thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared_data.n1.lock().unwrap();
    let n2 = shared_data.n2.lock().unwrap();
    println!("{} {}", n1, n2);

    0
}

fn main() {
    std::process::exit(main_0());
}