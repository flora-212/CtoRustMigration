use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn f1(shared: Arc<SharedData>) {
    let mut x = 0;
    let mut num_mutex = shared.num_mutex.lock().unwrap();
    while shared.n1 < 10 {
        x = shared.n1;
        *num_mutex = x;
        drop(num_mutex);
        x += 1;
        num_mutex = shared.num_mutex.lock().unwrap();
        *num_mutex = x;
    }
}

fn f2(shared: Arc<SharedData>) {
    let mut num_mutex = shared.num_mutex.lock().unwrap();
    while shared.n1 < 20 {
        if shared.n1 > 18 {
            drop(num_mutex);
            return;
        }
        *num_mutex += 1;
    }
}

fn t_fun(shared: Arc<SharedData>) {
    f1(shared.clone());
    f2(shared);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(0),
    });

    let shared_clone1 = shared_data.clone();
    let shared_clone2 = shared_data.clone();

    let handle1 = thread::spawn(move || t_fun(shared_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", shared_data.n1);

    0
}

fn main() {
    std::process::exit(main_0());
}
