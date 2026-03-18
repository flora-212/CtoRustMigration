use std::sync::{Arc, Mutex};
use std::thread;

pub struct st {
    n1: i32,
    num_mutex: Arc<Mutex<i32>>,
}

static mut s1: st = st {
    n1: 0,
    num_mutex: Arc::new(Mutex::new(0)),
};

static mut s2: st = st {
    n1: 1,
    num_mutex: Arc::new(Mutex::new(0)),
};

static mut s3: st = st {
    n1: 2,
    num_mutex: Arc::new(Mutex::new(0)),
};

fn f(s: &st) {
    let mut num_mutex = s.num_mutex.lock().unwrap();
    *num_mutex += 1;
}

fn f1() {
    unsafe {
        f(&s1);
        f(&s2);
        f(&s3);
    }
}

fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    let id1 = thread::spawn(|| unsafe { t_fun(std::ptr::null_mut()) });
    let id2 = thread::spawn(|| unsafe { t_fun(std::ptr::null_mut()) });

    id1.join().unwrap();
    id2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}