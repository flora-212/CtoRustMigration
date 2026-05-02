use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn lock(mutex: &Mutex<i32>) {
    let _guard = mutex.lock().unwrap();
}

fn unlock(_guard: std::sync::MutexGuard<i32>) {}

fn f1(shared_data: Arc<SharedData>) {
    let mut guard = shared_data.num_mutex.lock().unwrap();
    shared_data.n1 += 1;
}

fn lock2(shared_data: Arc<SharedData>, n: i32) -> i32 {
    let mut guard = shared_data.num_mutex.lock().unwrap();
    shared_data.n1 += n;
    shared_data.n1
}

fn unlock2(shared_data: Arc<SharedData>, n: i32) -> i32 {
    let mut guard = shared_data.num_mutex.lock().unwrap();
    shared_data.n1 += n;
    shared_data.n1
}

fn f2(shared_data: Arc<SharedData>) -> i32 {
    let n2 = lock2(shared_data.clone(), 1);
    shared_data.n1 += 1;
    unlock2(shared_data.clone(), 1)
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data.clone());
    f2(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(0),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let shared_data_clone = shared_data.clone();
        let handle = thread::spawn(move || {
            t_fun(shared_data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", shared_data.n1);
    0
}

fn main() {
    std::process::exit(main_0());
}