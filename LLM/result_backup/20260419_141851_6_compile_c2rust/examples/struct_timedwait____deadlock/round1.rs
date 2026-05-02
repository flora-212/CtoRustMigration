use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut SHARED_DATA: Option<Arc<SharedData>> = None;
static mut M2: Mutex<()> = Mutex::new(());

fn init_shared_data() {
    unsafe {
        SHARED_DATA = Some(Arc::new(SharedData {
            n1: 0,
            n2: 0,
            n3: 0,
            m1: Mutex::new(()),
            cond: std::sync::Condvar::new(),
        }));
    }
}

unsafe fn f1(shared_data: &Arc<SharedData>) {
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

unsafe fn f2(shared_data: &Arc<SharedData>) {
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

unsafe fn f3(shared_data: &Arc<SharedData>) {
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

unsafe fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared_data = SHARED_DATA.as_ref().unwrap();
    f1(shared_data);
    f2(shared_data);
    f3(shared_data);
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    init_shared_data();
    let shared_data = SHARED_DATA.as_ref().unwrap();

    let handle1 = thread::spawn(move || {
        t_fun(libc::NULL);
    });

    let handle2 = thread::spawn(move || {
        t_fun(libc::NULL);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", shared_data.n1, shared_data.n2, shared_data.n3);

    0
}

fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}