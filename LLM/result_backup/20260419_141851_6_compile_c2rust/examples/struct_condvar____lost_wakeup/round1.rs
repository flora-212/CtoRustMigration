use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedState {
    n1: i32,
    m1: Mutex<()>,
    cond: Arc<()>,
}

static mut S: Option<Arc<SharedState>> = None;

fn init_shared_state() -> Arc<SharedState> {
    unsafe {
        if S.is_none() {
            S = Some(Arc::new(SharedState {
                n1: 0,
                m1: Mutex::new(()),
                cond: Arc::new(()),
            }));
        }
        S.as_ref().unwrap().clone()
    }
}

fn f1(shared: Arc<SharedState>) {
    let mut m1 = shared.m1.lock().unwrap();
    shared.n1 += 1;
    if shared.n1 == 1 {
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

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, shared.n1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}