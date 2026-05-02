use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static SHARED_DATA: Arc<SharedData> = Arc::new(SharedData {
    n1: 0,
    n2: 0,
    n3: 0,
    m1: Mutex::new(()),
    cond: Condvar::new(),
});

fn f1(shared_data: Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let mut guard = shared_data.m1.lock().unwrap();
    shared_data.n1 += 1;
    if shared_data.n1 == 1 {
        ts += Duration::new(1, 0);
        guard = shared_data.cond.wait_until(guard, ts).unwrap();
    } else {
        shared_data.cond.notify_one();
    }
}

fn f2(shared_data: Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let mut guard = shared_data.m1.lock().unwrap();
    shared_data.n2 += 1;
    if shared_data.n2 == 1 {
        ts += Duration::new(0, 1_000_000_000);
        guard = shared_data.cond.wait_until(guard, ts).unwrap();
    } else {
        shared_data.cond.notify_one();
    }
}

fn f3(shared_data: Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let mut guard = shared_data.m1.lock().unwrap();
    shared_data.n3 += 1;
    if shared_data.n3 == 1 {
        ts += Duration::new(1, 2_000_000_000);
        guard = shared_data.cond.wait_until(guard, ts).unwrap();
    } else {
        shared_data.cond.notify_one();
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data.clone());
    f2(shared_data.clone());
    f3(shared_data);
}

fn main_0() -> i32 {
    let shared_data = SHARED_DATA.clone();

    let handle1 = thread::spawn(move || t_fun(shared_data.clone()));
    let handle2 = thread::spawn(move || t_fun(shared_data.clone()));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let shared_data = SHARED_DATA.clone();
    println!("{} {} {}", shared_data.n1, shared_data.n2, shared_data.n3);

    0
}

fn main() {
    std::process::exit(main_0());
}