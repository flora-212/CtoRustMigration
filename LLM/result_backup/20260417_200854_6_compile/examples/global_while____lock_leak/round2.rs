use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    num_mutex: Mutex<i32>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut x = 0;
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    let mut n1 = shared_data.n1.lock().unwrap();
    while *n1 < 10 {
        x = *n1;
        *num_mutex = x;
        drop(num_mutex);
        x += 1;
        num_mutex = shared_data.num_mutex.lock().unwrap();
        *n1 = x;
    }
}

fn f2(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    let mut n1 = shared_data.n1.lock().unwrap();
    while *n1 < 20 {
        if *n1 > 18 {
            drop(num_mutex);
            return;
        }
        *n1 += 1;
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data.clone());
    f2(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
        num_mutex: Mutex::new(0),
    });

    let shared_data_clone1 = shared_data.clone();
    let handle1 = thread::spawn(move || t_fun(shared_data_clone1));

    let shared_data_clone2 = shared_data.clone();
    let handle2 = thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", *shared_data.n1.lock().unwrap());
    0
}

fn main() {
    std::process::exit(main_0());
}