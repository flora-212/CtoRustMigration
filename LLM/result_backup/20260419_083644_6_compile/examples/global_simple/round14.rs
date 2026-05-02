use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    num_mutex: Mutex<i32>,
}

fn f1(shared_data: Arc<SharedData>) {
    let x = {
        let data = shared_data.lock().unwrap();
        data.n3
    };
    let mut data = shared_data.lock().unwrap();
    data.n1 += x;
    data.n2 += x;
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        n3: 1,
        num_mutex: Mutex::new(0),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || t_fun(shared_data_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = shared_data.lock().unwrap();
    println!("{} {} {}", data.n1, data.n2, data.n3);

    0
}

fn main() {
    std::process::exit(main_0());
}