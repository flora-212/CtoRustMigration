use std::sync::{Arc, Mutex};
use std::thread;

pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Arc<Mutex<()>>,
    m2: Arc<Mutex<()>>,
}

fn main() {
    let m1 = Arc::new(Mutex::new(()));
    let m2 = Arc::new(Mutex::new(()));

    let shared_data = Arc::new(SharedData {
        n1: 0,
        n2: 1,
        n3: 2,
        n4: 3,
        m1: m1.clone(),
        m2: m2.clone(),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let data_clone = shared_data.clone();
        let handle = thread::spawn(move || t_fun(data_clone));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn f1(shared_data: &SharedData) {
    let x = shared_data.n4;
    let mut m1 = shared_data.m1.lock().unwrap();
    shared_data.n1 += x;
    shared_data.n2 += x;
    drop(m1);
    let mut m2 = shared_data.m2.lock().unwrap();
    shared_data.n3 += x;
    drop(m2);
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(&shared_data);
}
