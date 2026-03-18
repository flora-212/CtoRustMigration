use ::libc;
use std::ptr;
use std::sync::{Mutex, Arc};
use std::thread;

#[derive(Copy, Clone)]
pub struct ss {
    pub n: i32,
    pub m: Arc<Mutex<()>>,
}

static mut s1: ss = ss {
    n: 0,
    m: Arc::new(Mutex::new(())),
};

static mut s2: ss = ss {
    n: 0,
    m: Arc::new(Mutex::new(())),
};

fn f1(s: &ss) {
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

fn t_fun(arg: &ss) {
    f1(&s1);
    f1(&s2);
    f1(arg);
}

unsafe fn main_0() -> libc::c_int {
    let s3 = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });

    let s3_clone = Arc::clone(&s3);

    let handle1 = thread::spawn(move || {
        t_fun(&s3);
    });

    let handle2 = thread::spawn(move || {
        t_fun(&s3_clone);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    return 0 as libc::c_int;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}