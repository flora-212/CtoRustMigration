use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    num_mutex: Mutex<()>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut num_mutex = shared_data.num_mutex.lock().unwrap();
    let n2 = *shared_data.n2.lock().unwrap();
    *shared_data.n1.lock().unwrap() += n2;
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: Mutex::new(0),
        n2: Mutex::new(0),
        num_mutex: Mutex::new(()),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || t_fun(shared_data_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {}", *shared_data.n1.lock().unwrap(), *shared_data.n2.lock().unwrap());

    0
}

fn main() {
    std::process::exit(main_0());
}