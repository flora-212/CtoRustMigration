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

static mut S: Option<Arc<SharedData>> = None;
static mut M2: Mutex<()> = Mutex::new(());

fn initialize_shared_data() {
    unsafe {
        S = Some(Arc::new(SharedData {
            n1: 0,
            n2: 0,
            n3: 0,
            m1: Mutex::new(()),
            cond: std::sync::Condvar::new(),
        }));
    }
}

unsafe fn f1() {
    let s = S.as_ref().unwrap();
    let mut m2 = M2.lock().unwrap();
    let mut m1 = s.m1.lock().unwrap();

    s.n1 += 1;
    if s.n1 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &mut ts);
        ts.tv_sec += 1;
        let deadline = Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        let _ = s.cond.wait_timeout(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

unsafe fn f2() {
    let s = S.as_ref().unwrap();
    let mut m2 = M2.lock().unwrap();
    let mut m1 = s.m1.lock().unwrap();

    s.n2 += 1;
    if s.n2 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &mut ts);
        ts.tv_nsec += 1_000_000_000; // 1 second in nanoseconds
        let deadline = Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        let _ = s.cond.wait_timeout(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

unsafe fn f3() {
    let s = S.as_ref().unwrap();
    let mut m1 = s.m1.lock().unwrap();

    s.n3 += 1;
    if s.n3 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &mut ts);
        ts.tv_sec += 1;
        ts.tv_nsec += 2_000_000_000; // 2 seconds in nanoseconds
        let deadline = Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        let _ = s.cond.wait_timeout(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

unsafe fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    initialize_shared_data();
    let s = S.as_ref().unwrap();

    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = s.lock().unwrap();
    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}