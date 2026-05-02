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
}

fn f2(shared: Arc<SharedData>) {
    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);
    if let Err(_) = shared.m.try_lock() {
        return;
    }
    *shared.m.lock().unwrap() += 1;
}

fn f3(shared: Arc<SharedData>) {
    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);
    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);
    if let Err(_) = shared.m.try_lock() {
        return;
    }
    *shared.m.lock().unwrap() += 1;
}

fn f4(shared: Arc<SharedData>) {
    let mut m = shared.m.lock().unwrap();
    *m += 1;
    drop(m);
    if let Err(_) = shared.m.try_lock() {
        return;
    }
    *shared.m.lock().unwrap() += 1;
    drop(m);
    let mut m = shared.m.lock().unwrap();
    *m += 1;
}

fn t_fun(shared: Arc<SharedData>) {
    f1(shared.clone());
    f2(shared.clone());
    f3(shared.clone());
    f4(shared);
}

fn main_0() -> i32 {
    let shared = Arc::new(SharedData {
        n: 0,
        m: Mutex::new(0),
    });

    let handle1 = thread::spawn(move || t_fun(shared.clone()));
    let handle2 = thread::spawn(move || t_fun(shared));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", shared.n);
    0
}

fn main() {
    std::process::exit(main_0());
}