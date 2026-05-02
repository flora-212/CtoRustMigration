use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct SharedState {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut S: Option<Arc<SharedState>> = None;

fn init_shared_state() -> Arc<SharedState> {
    unsafe {
        if S.is_none() {
            S = Some(Arc::new(SharedState {
                n1: 0,
                n2: 0,
                n3: 0,
                m1: Mutex::new(()),
                cond: std::sync::Condvar::new(),
            }));
        }
        S.as_ref().unwrap().clone()
    }
}

fn f1(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut guard = shared.m1.lock().unwrap();
    shared.n1 += 1;
    ts += Duration::from_secs(1);
    let _ = shared.cond.wait_until(&mut guard, ts);
}

fn f2(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut guard = shared.m1.lock().unwrap();
    shared.n2 += 1;
    ts += Duration::from_nanos(1);
    let _ = shared.cond.wait_until(&mut guard, ts);
}

fn f3(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut guard = shared.m1.lock().unwrap();
    shared.n3 += 1;
    ts += Duration::from_secs(1) + Duration::from_nanos(2);
    let _ = shared.cond.wait_until(&mut guard, ts);
}

fn t_fun(shared: Arc<SharedState>) {
    f1(shared.clone());
    f2(shared.clone());
    f3(shared);
}

fn main_0() -> i32 {
    let shared = init_shared_state();

    let handle1 = thread::spawn(move || t_fun(shared.clone()));
    let handle2 = thread::spawn(move || t_fun(shared.clone()));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared.n1;
    let n2 = shared.n2;
    let n3 = shared.n3;

    println!("{} {} {}", n1, n2, n3);

    0
}

fn main() {
    std::process::exit(main_0());
}