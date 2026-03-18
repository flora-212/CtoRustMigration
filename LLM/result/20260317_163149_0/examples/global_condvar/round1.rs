use std::sync::{Arc, Condvar, Mutex};

static mut N1: i32 = 0;
static mut N2: i32 = 0;

struct SharedData {
    n1: i32,
    n2: i32,
    mutex: Mutex<()>,
    cond: Condvar,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut data = shared_data.lock().unwrap();
    unsafe {
        N1 += 1;
    }
    if unsafe { N1 } == 1 {
        data = shared_data.cond.wait(data).unwrap();
    } else {
        shared_data.cond.notify_one();
    }

    unsafe {
        N2 += 1;
    }
    if unsafe { N2 } == 1 {
        data = shared_data.cond.wait(data).unwrap();
    } else {
        shared_data.cond.notify_all();
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main() {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        mutex: Mutex::new(()),
        cond: Condvar::new(),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = std::thread::spawn(move || t_fun(shared_data_clone1));
    let handle2 = std::thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();
}