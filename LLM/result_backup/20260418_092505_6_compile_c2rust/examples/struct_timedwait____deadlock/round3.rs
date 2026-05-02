use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use libc::{clock_gettime, CLOCK_REALTIME, timespec};

#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::time_t,
    pub tv_nsec: libc::c_long,
}

#[derive(Debug)]
pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

lazy_static::lazy_static! {
    static ref S: Arc<SharedData> = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        n3: 0,
        m1: Mutex::new(()),
        cond: std::sync::Condvar::new(),
    });
}

static M2: Mutex<()> = Mutex::new(());

fn f1() {
    let s = S.clone();
    let mut m2 = M2.lock().unwrap();
    let mut m1 = s.m1.lock().unwrap();

    s.n1 += 1;
    if s.n1 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            clock_gettime(CLOCK_REALTIME, &mut ts);
        }
        ts.tv_sec += 1;
        let deadline = Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        let _ = s.cond.wait_timeout(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f2() {
    let s = S.clone();
    let mut m2 = M2.lock().unwrap();
    let mut m1 = s.m1.lock().unwrap();

    s.n2 += 1;
    if s.n2 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            clock_gettime(CLOCK_REALTIME, &mut ts);
        }
        ts.tv_nsec += 1_000_000_000; // 1 second in nanoseconds
        let deadline = Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        let _ = s.cond.wait_timeout(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f3() {
    let s = S.clone();
    let mut m1 = s.m1.lock().unwrap();

    s.n3 += 1;
    if s.n3 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            clock_gettime(CLOCK_REALTIME, &mut ts);
        }
        ts.tv_sec += 1;
        ts.tv_nsec += 2_000_000_000; // 2 seconds in nanoseconds
        let deadline = Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        let _ = s.cond.wait_timeout(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn t_fun() {
    f1();
    f2();
    f3();
}

fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.lock().unwrap();
    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}