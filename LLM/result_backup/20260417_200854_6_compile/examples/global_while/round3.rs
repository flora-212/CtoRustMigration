use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    num_mutex: Mutex<()>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut x = 0;
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    while *shared_data.n1.lock().unwrap() < 10 {
        x = *shared_data.n1.lock().unwrap();
        drop(num_mutex);
        x += 1;
        num_mutex = shared_data.num_mutex.lock().unwrap();
        *shared_data.n1.lock().unwrap() = x;
    }
}

fn f2(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    while *shared_data.n1.lock().unwrap() < 20 {
        if *shared_data.n1.lock().unwrap() > 18 {
            drop(num_mutex);
            return;
        }
        *shared_data.n1.lock().unwrap() += 1;
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data.clone());
    f2(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
        num_mutex: Mutex::new(()),
    });

    let shared_data_clone = shared_data.clone();
    let handle1 = thread::spawn(move || t_fun(shared_data_clone));

    let shared_data_clone = shared_data.clone();
    let handle2 = thread::spawn(move || t_fun(shared_data_clone));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", *shared_data.n1.lock().unwrap());
    0
}

fn main() {
    std::process::exit(main_0());
}