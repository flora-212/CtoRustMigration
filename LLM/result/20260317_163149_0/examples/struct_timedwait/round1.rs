use ::libc;
use std::sync::{Mutex, Condvar};
use std::time::{Duration, Instant};

extern "C" {
    fn clock_gettime(__clock_id: libc::c_int, __tp: *mut libc::timespec) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::time_t,
    pub tv_nsec: libc::c_long,
}

pub type clockid_t = libc::c_int;

#[derive(Default)]
pub struct SharedState {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

static mut S: SharedState = SharedState::default();

fn f1() {
    let mut s = unsafe { &mut S };
    let mut lock = s.m1.lock().unwrap();
    s.n1 += 1;
    if s.n1 == 1 {
        let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            clock_gettime(0, &mut ts);
        }
        ts.tv_sec += 1;
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        lock = s.cond.wait_until(lock, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f2() {
    let mut s = unsafe { &mut S };
    let mut lock = s.m1.lock().unwrap();
    s.n2 += 1;
    if s.n2 == 1 {
        let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            clock_gettime(0, &mut ts);
        }
        ts.tv_nsec += 1_000_000_000; // Convert nanoseconds to seconds
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        lock = s.cond.wait_until(lock, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f3() {
    let mut s = unsafe { &mut S };
    let mut lock = s.m1.lock().unwrap();
    s.n3 += 1;
    if s.n3 == 1 {
        let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            clock_gettime(0, &mut ts);
        }
        ts.tv_sec += 1;
        ts.tv_nsec += 2_000_000_000; // Convert nanoseconds to seconds
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        lock = s.cond.wait_until(lock, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn t_fun() {
    f1();
    f2();
    f3();
}

fn main() {
    let handle1 = std::thread::spawn(t_fun);
    let handle2 = std::thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();
}