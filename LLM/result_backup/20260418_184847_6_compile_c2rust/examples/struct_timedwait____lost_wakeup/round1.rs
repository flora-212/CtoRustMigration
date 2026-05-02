use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use libc::{clock_gettime, timespec, CLOCK_REALTIME};
use std::ffi::CString;
use std::ptr;

#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::time_t,
    pub tv_nsec: libc::c_long,
}

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
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut guard = shared.m1.lock().unwrap();
    shared.n1 += 1;
    unsafe {
        clock_gettime(CLOCK_REALTIME, &mut ts);
    }
    ts.tv_sec += 1;
    let wait_until = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
    let now = SystemTime::now();
    if now < wait_until {
        let duration = wait_until.duration_since(now).unwrap();
        guard = shared.cond.wait_timeout(guard, duration).unwrap().0;
    }
}

fn f2(shared: Arc<SharedState>) {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut guard = shared.m1.lock().unwrap();
    shared.n2 += 1;
    unsafe {
        clock_gettime(CLOCK_REALTIME, &mut ts);
    }
    ts.tv_nsec += 1_000_000_000; // 1 second in nanoseconds
    let wait_until = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
    let now = SystemTime::now();
    if now < wait_until {
        let duration = wait_until.duration_since(now).unwrap();
        guard = shared.cond.wait_timeout(guard, duration).unwrap().0;
    }
}

fn f3(shared: Arc<SharedState>) {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut guard = shared.m1.lock().unwrap();
    shared.n3 += 1;
    unsafe {
        clock_gettime(CLOCK_REALTIME, &mut ts);
    }
    ts.tv_sec += 1;
    ts.tv_nsec += 2_000_000_000; // 2 seconds in nanoseconds
    let wait_until = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
    let now = SystemTime::now();
    if now < wait_until {
        let duration = wait_until.duration_since(now).unwrap();
        guard = shared.cond.wait_timeout(guard, duration).unwrap().0;
    }
}

fn t_fun(shared: Arc<SharedState>) {
    f1(shared.clone());
    f2(shared.clone());
    f3(shared);
}

fn main_0() -> i32 {
    let shared = init_shared_state();
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(move || {
            t_fun(shared.clone());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let n1 = shared.n1;
    let n2 = shared.n2;
    let n3 = shared.n3;

    let c_string = CString::new(format!("{} {} {}\n", n1, n2, n3)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    std::process::exit(main_0());
}