use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Once;

static INIT: Once = Once::new();
static mut SHARED_DATA: Option<Arc<SharedData>> = None;

struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

fn initialize_shared_data() {
    INIT.call_once(|| {
        unsafe {
            SHARED_DATA = Some(Arc::new(SharedData {
                n1: 0,
                n2: 0,
                n3: 0,
                m1: Mutex::new(()),
                cond: Condvar::new(),
            }));
        }
    });
}

fn get_shared_data() -> Arc<SharedData> {
    unsafe { SHARED_DATA.as_ref().unwrap().clone() }
}

fn f1(shared_data: Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let mut guard = shared_data.m1.lock().unwrap();
    let n1 = *guard.n1 + 1;
    *guard.n1 = n1;
    if n1 == 1 {
        ts += Duration::new(1, 0);
        guard = shared_data.cond.wait(guard).unwrap();
    } else {
        shared_data.cond.notify_one();
    }
}

fn f2(shared_data: Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let mut guard = shared_data.m1.lock().unwrap();
    let n2 = *guard.n2 + 1;
    *guard.n2 = n2;
    if n2 == 1 {
        ts += Duration::new(0, 1_000_000_000);
        guard = shared_data.cond.wait(guard).unwrap();
    } else {
        shared_data.cond.notify_one();
    }
}

fn f3(shared_data: Arc<SharedData>) {
    let mut ts = SystemTime::now();
    let mut guard = shared_data.m1.lock().unwrap();
    let n3 = *guard.n3 + 1;
    *guard.n3 = n3;
    if n3 == 1 {
        ts += Duration::new(1, 2_000_000_000);
        guard = shared_data.cond.wait(guard).unwrap();
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
    initialize_shared_data();
    let shared_data = get_shared_data();

    let handle1 = thread::spawn(move || t_fun(shared_data.clone()));
    let handle2 = thread::spawn(move || t_fun(shared_data.clone()));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let shared_data = get_shared_data();
    println!("{} {} {}", shared_data.n1, shared_data.n2, shared_data.n3);

    0
}

fn main() {
    std::process::exit(main_0());
}