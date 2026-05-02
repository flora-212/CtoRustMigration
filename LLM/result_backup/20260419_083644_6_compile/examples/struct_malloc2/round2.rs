use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

#[no_mangle]
static mut X: Option<Arc<Ss>> = None;

#[no_mangle]
unsafe extern "C" fn f1() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(()),
    });
    X = Some(s.clone());
    let mut guard = s.m.lock().unwrap();
    let mut s = s.clone();
    s.n = 456;
    drop(guard);
}

#[no_mangle]
unsafe extern "C" fn f2() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(()),
    });
    X = Some(s.clone());
    let mut guard = s.m.lock().unwrap();
    let mut s = s.clone();
    s.n = 789;
    drop(guard);
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });
    let handle2 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    if let Some(ref x) = X {
        let n = x.n;
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}