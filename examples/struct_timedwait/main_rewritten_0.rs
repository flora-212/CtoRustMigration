use ::libc;
use std::sync::{Mutex, Condvar};
use std::time::{Duration, Instant};
use lazy_static::lazy_static;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::c_long,
    pub tv_nsec: libc::c_long,
}

pub type clockid_t = libc::c_int;

extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}

#[derive(Default)]
pub struct SharedState {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: Condvar,
}

lazy_static! {
    static ref S: SharedState = SharedState::default();
}

fn f1() {
    let mut s = S.m1.lock().unwrap();
    S.n1 += 1;
    if S.n1 == 1 {
        let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            clock_gettime(0, &mut ts);
        }
        let wait_time = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
        s = S.cond.wait_timeout(s, wait_time).unwrap().0;
    } else {
        S.cond.notify_one();
    }
}

fn f2() {
    let mut s = S.m1.lock().unwrap();
    S.n2 += 1;
    if S.n2 == 1 {
        let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            clock_gettime(0, &mut ts);
        }
        let wait_time = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
        s = S.cond.wait_timeout(s, wait_time).unwrap().0;
    } else {
        S.cond.notify_one();
    }
}

fn f3() {
    let mut s = S.m1.lock().unwrap();
    S.n3 += 1;
    if S.n3 == 1 {
        let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            clock_gettime(0, &mut ts);
        }
        let wait_time = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
        s = S.cond.wait_timeout(s, wait_time).unwrap().0;
    } else {
        S.cond.notify_one();
    }
}

fn t_fun() {
    f1();
    f2();
    f3();
}

fn main_0() -> libc::c_int {
    let handle1 = std::thread::spawn(t_fun);
    let handle2 = std::thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
