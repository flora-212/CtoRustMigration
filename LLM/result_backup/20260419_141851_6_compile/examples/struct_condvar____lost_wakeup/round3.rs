use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;
use std::sync::Once;

#[derive(Debug)]
struct SharedState {
    n1: Mutex<i32>,
    m1: Mutex<()>,
    cond: Arc<()>,
}

static mut S: Option<Arc<SharedState>> = None;
static INIT: Once = Once::new();

fn init_shared_state() -> Arc<SharedState> {
    INIT.call_once(|| {
        unsafe {
            S = Some(Arc::new(SharedState {
                n1: Mutex::new(0),
                m1: Mutex::new(()),
                cond: Arc::new(()),
            }));
        }
    });
    unsafe { S.as_ref().unwrap().clone() }
}

fn f1(shared: Arc<SharedState>) {
    let mut m1 = shared.m1.lock().unwrap();
    let mut n1 = shared.n1.lock().unwrap();
    *n1 += 1;
    if *n1 == 1 {
        thread::sleep(std::time::Duration::from_secs(1)); // Simulate waiting
    } else {
        thread::sleep(std::time::Duration::from_secs(1)); // Simulate waiting
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared = init_shared_state();
    f1(shared);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let shared = init_shared_state();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared.n1.lock().unwrap();
    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *n1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}