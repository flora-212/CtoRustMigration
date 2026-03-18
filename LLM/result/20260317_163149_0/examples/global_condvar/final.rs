use std::sync::{Arc, Condvar, Mutex};

struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    cond: Condvar,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut n1 = shared_data.n1.lock().unwrap();
    *n1 += 1;
    if *n1 == 1 {
        n1 = shared_data.cond.wait(n1).unwrap();
    } else {
        shared_data.cond.notify_one();
    }

    let mut n2 = shared_data.n2.lock().unwrap();
    *n2 += 1;
    if *n2 == 1 {
        n2 = shared_data.cond.wait(n2).unwrap();
    } else {
        shared_data.cond.notify_all();
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main() {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
        n2: Mutex::new(0),
        cond: Condvar::new(),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = std::thread::spawn(move || t_fun(shared_data_clone1));
    let handle2 = std::thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();
}
