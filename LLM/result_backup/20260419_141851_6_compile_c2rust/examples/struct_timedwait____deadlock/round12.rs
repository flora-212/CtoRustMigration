use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::Once;
use std::sync::OnceLock;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static SHARED_DATA: OnceLock<Arc<SharedData>> = OnceLock::new();
static M2: Mutex<()> = Mutex::new(());

fn init_shared_data() {
    SHARED_DATA.set(Arc::new(SharedData {
        n1: 0,
        n2: 0,
        n3: 0,
        m1: Mutex::new(()),
        cond: std::sync::Condvar::new(),
    })).unwrap();
}

fn f1(shared_data: &Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let _guard1 = shared_data.m1.lock().unwrap();
    let _guard2 = M2.lock().unwrap();
    shared_data.n1 += 1;
    if shared_data.n1 == 1 {
        ts = ts + Duration::from_secs(1);
        let result = shared_data.cond.wait_until(&mut _guard1, ts).unwrap();
        if result.timed_out() {
            println!("Timed out in f1");
        }
    } else {
        shared_data.cond.notify_one();
    }
}

fn f2(shared_data: &Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let _guard2 = M2.lock().unwrap();
    let _guard1 = shared_data.m1.lock().unwrap();
    shared_data.n2 += 1;
    if shared_data.n2 == 1 {
        ts = ts + Duration::from_nanos(1);
        let result = shared_data.cond.wait_until(&mut _guard1, ts).unwrap();
        if result.timed_out() {
            println!("Timed out in f2");
        }
    } else {
        shared_data.cond.notify_one();
    }
}

fn f3(shared_data: &Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let _guard1 = shared_data.m1.lock().unwrap();
    shared_data.n3 += 1;
    if shared_data.n3 == 1 {
        ts = ts + Duration::new(1, 2);
        let result = shared_data.cond.wait_until(&mut _guard1, ts).unwrap();
        if result.timed_out() {
            println!("Timed out in f3");
        }
    } else {
        shared_data.cond.notify_one();
    }
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(&shared_data);
    f2(&shared_data);
    f3(&shared_data);
}

fn main_0() -> i32 {
    init_shared_data();
    let shared_data = SHARED_DATA.get().unwrap().clone();

    let handle1 = thread::spawn(move || {
        t_fun(shared_data.clone());
    });

    let handle2 = thread::spawn(move || {
        t_fun(shared_data.clone());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", SHARED_DATA.get().unwrap().n1, SHARED_DATA.get().unwrap().n2, SHARED_DATA.get().unwrap().n3);

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}