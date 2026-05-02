use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::OnceLock;

#[derive(Debug)]
struct SharedState {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static SHARED_STATE: OnceLock<Arc<SharedState>> = OnceLock::new();

fn f1() {
    let state = SHARED_STATE.get().unwrap().clone();
    let mut guard = state.m1.lock().unwrap();
    state.n1 += 1;
    if state.n1 == 1 {
        let now = Instant::now();
        let wait_until = now + Duration::from_secs(1);
        let result = state.cond.wait_until(&mut guard, || Instant::now() < wait_until).unwrap();
        if result.timed_out() {
            println!("Timed out waiting for f1");
        }
    } else {
        state.cond.notify_one();
    }
}

fn f2() {
    let state = SHARED_STATE.get().unwrap().clone();
    let mut guard = state.m1.lock().unwrap();
    state.n2 += 1;
    if state.n2 == 1 {
        let now = Instant::now();
        let wait_until = now + Duration::from_nanos(1);
        let result = state.cond.wait_until(&mut guard, || Instant::now() < wait_until).unwrap();
        if result.timed_out() {
            println!("Timed out waiting for f2");
        }
    } else {
        state.cond.notify_one();
    }
}

fn f3() {
    let state = SHARED_STATE.get().unwrap().clone();
    let mut guard = state.m1.lock().unwrap();
    state.n3 += 1;
    if state.n3 == 1 {
        let now = Instant::now();
        let wait_until = now + Duration::new(1, 2);
        let result = state.cond.wait_until(&mut guard, || Instant::now() < wait_until).unwrap();
        if result.timed_out() {
            println!("Timed out waiting for f3");
        }
    } else {
        state.cond.notify_one();
    }
}

fn t_fun() {
    f1();
    f2();
    f3();
}

fn main_0() -> i32 {
    SHARED_STATE.get_or_init(|| {
        Arc::new(SharedState {
            n1: 0,
            n2: 0,
            n3: 0,
            m1: Mutex::new(()),
            cond: Condvar::new(),
        })
    });

    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();

    let state = SHARED_STATE.get().unwrap().lock().unwrap();
    println!("{} {} {}", state.n1, state.n2, state.n3);

    0
}

fn main() {
    std::process::exit(main_0());
}