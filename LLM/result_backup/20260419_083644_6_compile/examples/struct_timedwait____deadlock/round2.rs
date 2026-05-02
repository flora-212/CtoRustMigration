use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use std::ffi::CString;
use libc::{clock_gettime, timespec, CLOCK_REALTIME};

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    n3: Mutex<i32>,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static SHARED_DATA: Arc<SharedData> = Arc::new(SharedData {
    n1: Mutex::new(0),
    n2: Mutex::new(0),
    n3: Mutex::new(0),
    m1: Mutex::new(()),
    cond: std::sync::Condvar::new(),
});

fn f1(shared_data: Arc<SharedData>) {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut m1 = shared_data.m1.lock().unwrap();
    *shared_data.n1.lock().unwrap() += 1;
    if *shared_data.n1.lock().unwrap() == 1 {
        unsafe {
            clock_gettime(CLOCK_REALTIME, &mut ts);
        }
        ts.tv_sec += 1;
        let result = shared_data.cond.wait_timeout(m1, Duration::from_secs(ts.tv_sec as u64)).unwrap();
        m1 = result.0;
    } else {
        shared_data.cond.notify_one();
    }
}

fn f2(shared_data: Arc<SharedData>) {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut m1 = shared_data.m1.lock().unwrap();
    *shared_data.n2.lock().unwrap() += 1;
    if *shared_data.n2.lock().unwrap() == 1 {
        unsafe {
            clock_gettime(CLOCK_REALTIME, &mut ts);
        }
        ts.tv_nsec += 1_000_000_000; // 1 second in nanoseconds
        let result = shared_data.cond.wait_timeout(m1, Duration::from_nanos(ts.tv_nsec as u64)).unwrap();
        m1 = result.0;
    } else {
        shared_data.cond.notify_one();
    }
}

fn f3(shared_data: Arc<SharedData>) {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut m1 = shared_data.m1.lock().unwrap();
    *shared_data.n3.lock().unwrap() += 1;
    if *shared_data.n3.lock().unwrap() == 1 {
        unsafe {
            clock_gettime(CLOCK_REALTIME, &mut ts);
        }
        ts.tv_sec += 1;
        ts.tv_nsec += 2_000_000_000; // 2 seconds in nanoseconds
        let result = shared_data.cond.wait_timeout(m1, Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64)).unwrap();
        m1 = result.0;
    } else {
        shared_data.cond.notify_one();
    }
}

fn t_fun() {
    let shared_data = SHARED_DATA.clone();
    f1(shared_data.clone());
    f2(shared_data.clone());
    f3(shared_data);
}

fn main_0() -> i32 {
    let handle1 = thread::spawn(move || {
        t_fun();
    });

    let handle2 = thread::spawn(move || {
        t_fun();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = *SHARED_DATA.n1.lock().unwrap();
    let n2 = *SHARED_DATA.n2.lock().unwrap();
    let n3 = *SHARED_DATA.n3.lock().unwrap();

    let c_string = CString::new(format!("{} {} {}\n", n1, n2, n3)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    std::process::exit(main_0());
}