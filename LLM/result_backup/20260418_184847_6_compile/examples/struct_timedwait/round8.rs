use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::sync::OnceLock;

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[derive(Debug)]
struct SharedState {
    n1: i32,
    n2: i32,
    n3: i32,
    m1: Mutex<()>,
    cond: std::sync::Condvar,
}

static SHARED_STATE: OnceLock<Arc<Mutex<SharedState>>> = OnceLock::new();

unsafe extern "C" fn f1() {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let state = SHARED_STATE.get().unwrap().clone();
    let mut guard = state.lock().unwrap();
    guard.n1 += 1;
    if guard.n1 == 1 {
        ts = get_current_time();
        ts.tv_sec += 1;
        drop(guard);
        let mut state = SHARED_STATE.get().unwrap().lock().unwrap();
        guard = SHARED_STATE.get().unwrap().lock().unwrap().cond.wait_timeout(guard, Duration::from_nanos((ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64))).unwrap().0;
    } else {
        SHARED_STATE.get().unwrap().lock().unwrap().cond.notify_one();
    }
}

unsafe extern "C" fn f2() {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let state = SHARED_STATE.get().unwrap().clone();
    let mut guard = state.lock().unwrap();
    guard.n2 += 1;
    if guard.n2 == 1 {
        ts = get_current_time();
        ts.tv_nsec += 1_000_000_000; // 1 second in nanoseconds
        drop(guard);
        let mut state = SHARED_STATE.get().unwrap().lock().unwrap();
        guard = SHARED_STATE.get().unwrap().lock().unwrap().cond.wait_timeout(guard, Duration::from_nanos((ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64))).unwrap().0;
    } else {
        SHARED_STATE.get().unwrap().lock().unwrap().cond.notify_one();
    }
}

unsafe extern "C" fn f3() {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let state = SHARED_STATE.get().unwrap().clone();
    let mut guard = state.lock().unwrap();
    guard.n3 += 1;
    if guard.n3 == 1 {
        ts = get_current_time();
        ts.tv_sec += 1;
        ts.tv_nsec += 2_000_000_000; // 2 seconds in nanoseconds
        drop(guard);
        let mut state = SHARED_STATE.get().unwrap().lock().unwrap();
        guard = SHARED_STATE.get().unwrap().lock().unwrap().cond.wait_timeout(guard, Duration::from_nanos((ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64))).unwrap().0;
    } else {
        SHARED_STATE.get().unwrap().lock().unwrap().cond.notify_one();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    f2();
    f3();
    std::ptr::null_mut()
}

fn get_current_time() -> timespec {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    timespec {
        tv_sec: duration.as_secs() as i64,
        tv_nsec: duration.subsec_nanos() as i64,
    }
}

unsafe fn main_0() -> c_int {
    SHARED_STATE.get_or_init(|| {
        Arc::new(Mutex::new(SharedState {
            n1: 0,
            n2: 0,
            n3: 0,
            m1: Mutex::new(()),
            cond: std::sync::Condvar::new(),
        }))
    });

    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });
    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let state = SHARED_STATE.get().unwrap().lock().unwrap();
    let output = CString::new(format!("{} {} {}\n", state.n1, state.n2, state.n3)).unwrap();
    unsafe {
        libc::printf(output.as_ptr());
    }
    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}