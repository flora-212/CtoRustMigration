use std::sync::{Arc, Mutex};
use std::thread;

pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Arc<Mutex<i32>>,
    m2: Arc<Mutex<i32>>,
}

fn create_shared_data() -> SharedData {
    SharedData {
        n1: 0,
        n2: 1,
        n3: 2,
        n4: 3,
        m1: Arc::new(Mutex::new(0)),
        m2: Arc::new(Mutex::new(0)),
    }
}

fn f1(shared: &SharedData) {
    let x = shared.n4;
    {
        let mut lock = shared.m1.lock().unwrap();
        *lock += x;
    }
    {
        let mut lock = shared.m2.lock().unwrap();
        *lock += x;
    }
}

fn t_fun(shared: Arc<SharedData>) {
    f1(&shared);
}

fn main() {
    let shared = Arc::new(create_shared_data());
    let shared_clone1 = Arc::clone(&shared);
    let shared_clone2 = Arc::clone(&shared);

    let handle1 = thread::spawn(move || t_fun(shared_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();
}