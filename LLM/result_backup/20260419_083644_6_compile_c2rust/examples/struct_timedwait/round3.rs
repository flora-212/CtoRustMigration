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

impl SharedState {
    fn new() -> Self {
        SharedState {
            n1: 0,
            n2: 0,
            n3: 0,
            m1: Mutex::new(()),
            cond: std::sync::Condvar::new(),
        }
    }

    fn f1(&mut self) {
        let mut ts = SystemTime::now();
        let mut guard = self.m1.lock().unwrap();
        self.n1 += 1;
        if self.n1 == 1 {
            ts += Duration::from_secs(1);
            let result = self.cond.wait_until(&mut guard, ts);
            if result.is_err() {
                eprintln!("Condition wait timed out");
            }
        } else {
            self.cond.notify_one();
        }
    }

    fn f2(&mut self) {
        let mut ts = SystemTime::now();
        let mut guard = self.m1.lock().unwrap();
        self.n2 += 1;
        if self.n2 == 1 {
            ts += Duration::from_nanos(1);
            let result = self.cond.wait_until(&mut guard, ts);
            if result.is_err() {
                eprintln!("Condition wait timed out");
            }
        } else {
            self.cond.notify_one();
        }
    }

    fn f3(&mut self) {
        let mut ts = SystemTime::now();
        let mut guard = self.m1.lock().unwrap();
        self.n3 += 1;
        if self.n3 == 1 {
            ts += Duration::from_secs(1) + Duration::from_nanos(2);
            let result = self.cond.wait_until(&mut guard, ts);
            if result.is_err() {
                eprintln!("Condition wait timed out");
            }
        } else {
            self.cond.notify_one();
        }
    }
}

fn t_fun(shared: Arc<Mutex<SharedState>>) {
    let mut state = shared.lock().unwrap();
    state.f1();
    state.f2();
    state.f3();
}

fn main() {
    let shared_state = Arc::new(Mutex::new(SharedState::new()));

    let id1 = {
        let shared = Arc::clone(&shared_state);
        thread::spawn(move || t_fun(shared))
    };

    let id2 = {
        let shared = Arc::clone(&shared_state);
        thread::spawn(move || t_fun(shared))
    };

    id1.join().unwrap();
    id2.join().unwrap();

    let state = shared_state.lock().unwrap();
    println!("{} {} {}", state.n1, state.n2, state.n3);
}