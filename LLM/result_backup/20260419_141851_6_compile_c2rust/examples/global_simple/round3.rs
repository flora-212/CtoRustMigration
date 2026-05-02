use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    num_mutex: Mutex<()>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut x = {
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
        num_mutex: Mutex::new(()),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let shared_data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            t_fun(shared_data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = shared_data.lock().unwrap();
    println!("{} {} {}", data.n1, data.n2, data.n3);

    0
}

fn main() {
    std::process::exit(main_0());
}