use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Arc<Mutex<i32>>,
    m2: Arc<Mutex<i32>>,
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            n1: 0,
            n2: 1,
            n3: 2,
            n4: 3,
            m1: Arc::new(Mutex::new(0)),
            m2: Arc::new(Mutex::new(0)),
        }
    }

    pub fn f1(&mut self) {
        let x = self.n4;
        *self.m1.lock().unwrap() += x;
        *self.m2.lock().unwrap() += x;
    }
}

fn t_fun(shared_data: Arc<Mutex<SharedData>>) {
    let mut data = shared_data.lock().unwrap();
    data.f1();
}

fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData::new()));

    let handle1 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        move || t_fun(shared_data)
    });

    let handle2 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        move || t_fun(shared_data)
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
