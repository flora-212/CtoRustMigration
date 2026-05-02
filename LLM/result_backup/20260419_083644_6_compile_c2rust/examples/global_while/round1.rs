use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut x = 0;
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    while shared_data.n1 < 10 {
        x = shared_data.n1;
        *num_mutex = x;
        x += 1;
        drop(num_mutex);
        num_mutex = shared_data.num_mutex.lock().unwrap();
        shared_data.n1 = x;
    }
}

fn f2(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    while shared_data.n1 < 20 {
        if shared_data.n1 > 18 {
            drop(num_mutex);
            return;
        }
        shared_data.n1 += 1;
    }
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

    let shared_data_clone1 = shared_data.clone();
    let shared_data_clone2 = shared_data.clone();

    let handle1 = thread::spawn(move || t_fun(shared_data_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", shared_data.n1);
    0
}

fn main() {
    std::process::exit(main_0());
}