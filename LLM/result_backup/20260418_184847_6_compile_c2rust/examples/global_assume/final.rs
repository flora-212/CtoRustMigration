use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn inc(shared_data: &Arc<SharedData>) {
    let mut n1 = shared_data.num_mutex.lock().unwrap();
    *n1 += 1;
}

fn f1(shared_data: &Arc<SharedData>) {
    inc(shared_data);
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(&shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(0),
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

    println!("{}", shared_data.n1);
    0
}

fn main() {
    std::process::exit(main_0());
}
