use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::CStr;
use libc;

#[derive(Debug)]
pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<i32>,
    m2: Mutex<i32>,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            n1: 0,
            n2: 1,
            n3: 2,
            n4: 3,
            m1: Mutex::new(0),
            m2: Mutex::new(0),
        }
    }
}

static mut S: Option<Arc<SharedData>> = None;
static INIT: Once = Once::new();

fn initialize_shared_data() {
    unsafe {
        S = Some(Arc::new(SharedData::new()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s = S.as_ref().unwrap();
    let x = s.n4;
    let mut m1 = s.m1.lock().unwrap();
    *m1 += x;
    s.n1 += x;
    s.n2 += x;
    drop(m1);
    let mut m2 = s.m2.lock().unwrap();
    *m2 += x;
    s.n3 += x;
    s.n4 += x;
    drop(m2);
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(initialize_shared_data);

    let s = S.as_ref().unwrap().clone();
    let handle1 = thread::spawn(move || {
        f1();
    });
    let s = S.as_ref().unwrap().clone();
    let handle2 = thread::spawn(move || {
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(
        b"%d %d %d %d\n\0".as_ptr() as *const libc::c_char,
        S.as_ref().unwrap().n1,
        S.as_ref().unwrap().n2,
        S.as_ref().unwrap().n3,
        S.as_ref().unwrap().n4,
    );

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}