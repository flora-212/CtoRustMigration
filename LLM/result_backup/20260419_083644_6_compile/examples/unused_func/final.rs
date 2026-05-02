use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    *num_mutex += 1;
}

fn f2(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    *num_mutex += 1;
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(0),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

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
