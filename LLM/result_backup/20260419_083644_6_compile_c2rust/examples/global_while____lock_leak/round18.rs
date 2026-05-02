use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    num_mutex: Mutex<i32>,
}

fn f1(shared: Arc<SharedData>) {
    let mut x = 0;
    let mut num_mutex = shared.num_mutex.lock().unwrap();
    while *shared.n1.lock().unwrap() < 10 {
        x = *shared.n1.lock().unwrap();
        *num_mutex = x;
        drop(num_mutex);
        x += 1;
        num_mutex = shared.num_mutex.lock().unwrap();
        *shared.n1.lock().unwrap() = x;
    }
}

fn f2(shared: Arc<SharedData>) {
    let mut num_mutex = shared.num_mutex.lock().unwrap();
    while *shared.n1.lock().unwrap() < 20 {
        if *shared.n1.lock().unwrap() > 18 {
            drop(num_mutex);
            return;
        }
        *shared.n1.lock().unwrap() += 1;
    }
}

fn t_fun(shared: Arc<SharedData>) {
    f1(shared.clone());
    f2(shared);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
        num_mutex: Mutex::new(0),
    });

    let id1 = thread::spawn(move || t_fun(shared_data.clone()));
    let id2 = thread::spawn(move || t_fun(shared_data.clone()));

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{}", *shared_data.n1.lock().unwrap());
    0
}

fn main() {
    std::process::exit(main_0());
}