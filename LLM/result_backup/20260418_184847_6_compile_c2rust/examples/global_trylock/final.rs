use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n: Mutex<i32>,
    m: Mutex<()>,
}

fn f1(shared_data: Arc<SharedData>) {
    let mut m = shared_data.m.lock().unwrap();
    let mut n = shared_data.n.lock().unwrap();
    *n += 1;
    drop(n);
    drop(m);

    m = shared_data.m.lock().unwrap();
    n = shared_data.n.lock().unwrap();
    *n += 1;
    drop(n);
    drop(m);

    m = shared_data.m.lock().unwrap();
    n = shared_data.n.lock().unwrap();
    *n += 1;
    drop(n);
    drop(m);

    m = shared_data.m.lock().unwrap();
    n = shared_data.n.lock().unwrap();
    *n += 1;
    drop(n);
    drop(m);
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n: Mutex::new(0),
        m: Mutex::new(()),
    });

    let shared_data_clone1 = Arc::clone(&shared_data);
    let handle1 = thread::spawn(move || t_fun(shared_data_clone1));

    let shared_data_clone2 = Arc::clone(&shared_data);
    let handle2 = thread::spawn(move || t_fun(shared_data_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", *shared_data.n.lock().unwrap());
    0
}

fn main() {
    std::process::exit(main_0());
}
