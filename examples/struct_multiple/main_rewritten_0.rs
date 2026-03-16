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

fn f(s: &mut st) {
    let mut num_mutex = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

fn f1() {
    unsafe {
        f(&mut s1);
        f(&mut s2);
        f(&mut s3);
    }
}

fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

fn main_0() -> i32 {
    let t1 = thread::spawn(|| unsafe { t_fun(std::ptr::null_mut()) });
    let t2 = thread::spawn(|| unsafe { t_fun(std::ptr::null_mut()) });

    t1.join().unwrap();
    t2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
