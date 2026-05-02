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
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static mut SHARED_DATA: Option<Arc<SharedData>> = None;

unsafe fn initialize_shared_data() {
    SHARED_DATA = Some(Arc::new(SharedData {
        n1: 0,
        n2: 0,
        n3: 0,
        m1: Mutex::new(()),
        cond: std::sync::Condvar::new(),
    }));
}

unsafe fn get_shared_data() -> &'static Arc<SharedData> {
    SHARED_DATA.as_ref().unwrap()
}

unsafe extern "C" fn f1() {
    let shared_data = get_shared_data();
    let mut guard = shared_data.m1.lock().unwrap();
    shared_data.n1 += 1;
    if shared_data.n1 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &mut ts);
        ts.tv_sec += 1;
        guard = shared_data.cond.wait_timeout(guard, Duration::from_secs(ts.tv_sec as u64)).unwrap().0;
    } else {
        shared_data.cond.notify_one();
    }
}

unsafe extern "C" fn f2() {
    let shared_data = get_shared_data();
    let mut guard = shared_data.m1.lock().unwrap();
    shared_data.n2 += 1;
    if shared_data.n2 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &mut ts);
        ts.tv_nsec += 1_000_000_000; // 1 second in nanoseconds
        guard = shared_data.cond.wait_timeout(guard, Duration::from_nanos(ts.tv_nsec as u64)).unwrap().0;
    } else {
        shared_data.cond.notify_one();
    }
}

unsafe extern "C" fn f3() {
    let shared_data = get_shared_data();
    let mut guard = shared_data.m1.lock().unwrap();
    shared_data.n3 += 1;
    if shared_data.n3 == 1 {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &mut ts);
        ts.tv_sec += 1;
        ts.tv_nsec += 2_000_000_000; // 2 seconds in nanoseconds
        guard = shared_data.cond.wait_timeout(guard, Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64)).unwrap().0;
    } else {
        shared_data.cond.notify_one();
    }
}

unsafe extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    initialize_shared_data();
    let shared_data = get_shared_data();

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let shared_data = get_shared_data();
    let n1 = shared_data.n1;
    let n2 = shared_data.n2;
    let n3 = shared_data.n3;

    println!("{} {} {}", n1, n2, n3);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}