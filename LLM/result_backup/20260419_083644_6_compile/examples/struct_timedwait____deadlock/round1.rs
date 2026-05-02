use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use std::ffi::CString;
use libc::{clock_gettime, timespec, CLOCK_REALTIME};

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut SHARED_DATA: Option<Arc<SharedData>> = None;

unsafe fn get_shared_data() -> Arc<SharedData> {
    SHARED_DATA.get_or_insert_with(|| {
        Arc::new(SharedData {
            n1: 0,
            n2: 0,
            n3: 0,
            m1: Mutex::new(()),
            cond: std::sync::Condvar::new(),
        })
    }).clone()
}

fn f1(shared_data: Arc<SharedData>) {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut m1 = shared_data.m1.lock().unwrap();
    shared_data.n1 += 1;
    if shared_data.n1 == 1 {
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
    shared_data.n2 += 1;
    if shared_data.n2 == 1 {
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
    shared_data.n3 += 1;
    if shared_data.n3 == 1 {
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
    let shared_data = unsafe { get_shared_data() };
    f1(shared_data.clone());
    f2(shared_data.clone());
    f3(shared_data);
}

fn main_0() -> i32 {
    let shared_data = unsafe { get_shared_data() };

    let handle1 = thread::spawn(move || {
        t_fun();
    });

    let handle2 = thread::spawn(move || {
        t_fun();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared_data.n1;
    let n2 = shared_data.n2;
    let n3 = shared_data.n3;

    let c_string = CString::new(format!("{} {} {}\n", n1, n2, n3)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    std::process::exit(main_0());
}