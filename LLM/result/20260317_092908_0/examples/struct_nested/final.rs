use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;

#[derive(Copy, Clone)]
pub struct ss0 {
    pub n1: i32,
    pub n2: i32,
}

#[derive(Clone)]
pub struct ss {
    pub s: ss0,
    pub m: Arc<Mutex<()>>,
}

lazy_static! {
    static ref S: ss = ss {
        s: ss0 { n1: 0, n2: 1 },
        m: Arc::new(Mutex::new(())),
    };
}

fn f1() {
    let mut s = S.m.lock().unwrap();
    S.s.n1 += 1;
    S.s.n2 += 1;
}

extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    let s = S.clone();
    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let s = S.clone();
    let handle2 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
