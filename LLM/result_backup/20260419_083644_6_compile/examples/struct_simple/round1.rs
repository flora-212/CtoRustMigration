use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<()>,
    m2: Mutex<()>,
}

static mut S: Option<Arc<SharedData>> = None;

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s = S.as_ref().unwrap();
    let x = s.n4;

    let _guard1 = s.m1.lock().unwrap();
    s.n1 += x;
    s.n2 += x;

    let _guard2 = s.m2.lock().unwrap();
    s.n3 += x;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    let s = Arc::new(SharedData {
        n1: 0,
        n2: 1,
        n3: 2,
        n4: 3,
        m1: Mutex::new(()),
        m2: Mutex::new(()),
    });
    S = Some(s.clone());

    let s_clone1 = s.clone();
    let s_clone2 = s.clone();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.as_ref().unwrap();
    unsafe {
        libc::printf(
            b"%d %d %d %d\n\0".as_ptr(),
            s.n1,
            s.n2,
            s.n3,
            s.n4,
        );
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}