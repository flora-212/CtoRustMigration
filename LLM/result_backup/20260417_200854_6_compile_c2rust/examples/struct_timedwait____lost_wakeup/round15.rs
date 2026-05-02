use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

#[derive(Debug)]
struct SharedState {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    n3: Mutex<i32>,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut S: Option<Arc<SharedState>> = None;

fn initialize_shared_state() -> Arc<SharedState> {
    unsafe {
        if S.is_none() {
            S = Some(Arc::new(SharedState {
                n1: Mutex::new(0),
                n2: Mutex::new(0),
                n3: Mutex::new(0),
                m1: Mutex::new(()),
                cond: std::sync::Condvar::new(),
            }));
        }
        S.as_ref().unwrap().clone()
    }
}

fn f1(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut lock = shared.m1.lock().unwrap();
    *shared.n1.lock().unwrap() += 1;
    ts += Duration::from_secs(1);
    lock = shared.cond.wait(lock).unwrap();
}

fn f2(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut lock = shared.m1.lock().unwrap();
    *shared.n2.lock().unwrap() += 1;
    ts += Duration::from_nanos(1);
    lock = shared.cond.wait(lock).unwrap();
}

fn f3(shared: Arc<SharedState>) {
    let mut ts = SystemTime::now();
    let mut lock = shared.m1.lock().unwrap();
    *shared.n3.lock().unwrap() += 1;
    ts += Duration::from_secs(1) + Duration::from_nanos(2);
    lock = shared.cond.wait(lock).unwrap();
}

fn t_fun(shared: Arc<SharedState>) {
    f1(shared.clone());
    f2(shared.clone());
    f3(shared);
}

fn main_0() -> c_int {
    let shared = initialize_shared_state();

    let handle1 = thread::spawn(move || t_fun(shared.clone()));
    let handle2 = thread::spawn(move || t_fun(shared.clone()));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = *shared.n1.lock().unwrap();
    let n2 = *shared.n2.lock().unwrap();
    let n3 = *shared.n3.lock().unwrap();

    let output = format!("{} {} {}\n", n1, n2, n3);
    unsafe {
        libc::printf(CString::new(output).unwrap().as_ptr());
    }

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}