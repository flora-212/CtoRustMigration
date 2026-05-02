use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n: i32,
    m: Mutex<i32>,
}

fn f1(shared: Arc<SharedData>) {
    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);

    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);

    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);

    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);
}

fn t_fun(shared: Arc<SharedData>) {
    f1(shared);
}

fn main_0() -> i32 {
    let shared = Arc::new(SharedData {
        n: 0,
        m: Mutex::new(0),
    });

    let shared_clone1 = Arc::clone(&shared);
    let shared_clone2 = Arc::clone(&shared);

    let handle1 = thread::spawn(move || t_fun(shared_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", shared.n);
    0
}

fn main() {
    std::process::exit(main_0());
}