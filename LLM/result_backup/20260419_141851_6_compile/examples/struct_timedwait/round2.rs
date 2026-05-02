use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct SharedState {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static S: Arc<SharedState> = Arc::new(SharedState {
    n1: 0,
    n2: 0,
    n3: 0,
    m1: Mutex::new(()),
    cond: Condvar::new(),
});

fn f1(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut guard = shared.m1.lock().unwrap();
    shared.n1 += 1;
    if shared.n1 == 1 {
        ts += Duration::from_secs(1);
        guard = shared.cond.wait_until(guard, ts).unwrap();
    } else {
        shared.cond.notify_one();
    }
}

fn f2(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut guard = shared.m1.lock().unwrap();
    shared.n2 += 1;
    if shared.n2 == 1 {
        ts += Duration::from_nanos(1);
        guard = shared.cond.wait_until(guard, ts).unwrap();
    } else {
        shared.cond.notify_one();
    }
}

fn f3(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut guard = shared.m1.lock().unwrap();
    shared.n3 += 1;
    if shared.n3 == 1 {
        ts += Duration::from_secs(1) + Duration::from_nanos(2);
        guard = shared.cond.wait_until(guard, ts).unwrap();
    } else {
        shared.cond.notify_one();
    }
}

fn t_fun(shared: Arc<SharedState>) {
    f1(shared.clone());
    f2(shared.clone());
    f3(shared);
}

fn main_0() -> i32 {
    let id1 = thread::spawn(move || t_fun(S.clone()));
    let id2 = thread::spawn(move || t_fun(S.clone()));

    id1.join().unwrap();
    id2.join().unwrap();

    let n1 = S.n1;
    let n2 = S.n2;
    let n3 = S.n3;

    println!("{} {} {}", n1, n2, n3);

    0
}

fn main() {
    std::process::exit(main_0());
}