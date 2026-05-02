use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            n1: 0,
            n2: 0,
            n3: 0,
            m1: Mutex::new(()),
            cond: std::sync::Condvar::new(),
        }
    }
}

static S: OnceLock<Arc<Mutex<SharedData>>> = OnceLock::new();

fn f1(s: &Arc<Mutex<SharedData>>) {
    let mut ts = SystemTime::now();
    let mut guard = s.lock().unwrap();
    guard.n1 += 1;
    ts += Duration::new(1, 0);
    let result = s.cond.wait_until(&mut guard, ts);
    if result.is_err() {
        eprintln!("Condition wait error");
    }
}

fn f2(s: &Arc<Mutex<SharedData>>) {
    let mut ts = SystemTime::now();
    let mut guard = s.lock().unwrap();
    guard.n2 += 1;
    ts += Duration::new(0, 1_000_000_000);
    let result = s.cond.wait_until(&mut guard, ts);
    if result.is_err() {
        eprintln!("Condition wait error");
    }
}

fn f3(s: &Arc<Mutex<SharedData>>) {
    let mut ts = SystemTime::now();
    let mut guard = s.lock().unwrap();
    guard.n3 += 1;
    ts += Duration::new(1, 2_000_000_000);
    let result = s.cond.wait_until(&mut guard, ts);
    if result.is_err() {
        eprintln!("Condition wait error");
    }
}

fn t_fun(s: Arc<Mutex<SharedData>>) {
    f1(&s);
    f2(&s);
    f3(&s);
}

unsafe fn main_0() -> i32 {
    let s = S.get_or_init(|| Arc::new(Mutex::new(SharedData::new()))).clone();
    let s2 = S.get_or_init(|| Arc::new(Mutex::new(SharedData::new()))).clone();

    let handle1 = thread::spawn(move || t_fun(s));
    let handle2 = thread::spawn(move || t_fun(s2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.get_or_init(|| Arc::new(Mutex::new(SharedData::new()))).clone();
    let s = s.lock().unwrap();
    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}