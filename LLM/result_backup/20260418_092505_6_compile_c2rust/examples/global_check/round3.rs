use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexData {
    n: i32,
    m: Mutex<()>,
}

impl MutexData {
    fn new() -> Self {
        MutexData {
            n: 0,
            m: Mutex::new(()),
        }
    }

    fn increment(&mut self) {
        self.n += 1;
    }
}

fn f1(mutex_data: &Arc<MutexData>) {
    let mut guard = mutex_data.m.lock().unwrap();
    mutex_data.increment();
    drop(guard);
    guard = mutex_data.m.lock().unwrap();
    mutex_data.increment();
}

fn f2(mutex_data: &Arc<MutexData>) {
    let mut guard = mutex_data.m.lock().unwrap();
    mutex_data.increment();
    drop(guard);
    if let Err(_) = mutex_data.m.try_lock() {
        return;
    }
    mutex_data.increment();
}

fn f3(mutex_data: &Arc<MutexData>) {
    let mut guard = mutex_data.m.lock().unwrap();
    mutex_data.increment();
    drop(guard);
    if let Err(_) = mutex_data.m.lock() {
        return;
    }
    mutex_data.increment();
    if let Err(_) = mutex_data.m.try_lock() {
        return;
    }
    mutex_data.increment();
}

fn f4(mutex_data: &Arc<MutexData>) {
    let mut guard = mutex_data.m.lock().unwrap();
    mutex_data.increment();
    drop(guard);
    if let Err(_) = mutex_data.m.try_lock() {
        return;
    }
    mutex_data.increment();
    if let Err(_) = mutex_data.m.lock() {
        return;
    }
    mutex_data.increment();
}

fn t_fun(mutex_data: Arc<MutexData>) {
    f1(&mutex_data);
    f2(&mutex_data);
    f3(&mutex_data);
    f4(&mutex_data);
}

fn main_0() -> i32 {
    let mutex_data = Arc::new(MutexData::new());

    let id1 = thread::spawn({
        let mutex_data = Arc::clone(&mutex_data);
        move || t_fun(mutex_data)
    });

    let id2 = thread::spawn({
        let mutex_data = Arc::clone(&mutex_data);
        move || t_fun(mutex_data)
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{}", mutex_data.n);

    0
}

fn main() {
    std::process::exit(main_0());
}