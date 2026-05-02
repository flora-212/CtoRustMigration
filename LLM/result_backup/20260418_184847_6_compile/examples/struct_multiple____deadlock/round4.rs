use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use std::ptr;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<i32>,
}

#[no_mangle]
pub static S1: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 0,
    num_mutex: Mutex::new(0),
}));

#[no_mangle]
pub static S2: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 1,
    num_mutex: Mutex::new(0),
}));

#[no_mangle]
pub static S3: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 2,
    num_mutex: Mutex::new(0),
}));

#[no_mangle]
pub fn f(s: &Arc<Mutex<St>>, t: &Arc<Mutex<St>>) {
    let mut s_lock = s.lock().unwrap();
    let mut t_lock = t.lock().unwrap();
    s_lock.n1 = t_lock.n1 + 1;
    t_lock.n1 = s_lock.n1 + 1;
}

#[no_mangle]
pub fn f1() {
    f(&S1, &S2);
}

#[no_mangle]
pub fn f2() {
    f(&S2, &S3);
}

#[no_mangle]
pub fn f3() {
    f(&S1, &S3);
}

#[no_mangle]
pub fn t_fun(arg: isize) -> isize {
    if arg == 0 {
        f1();
    } else if arg == 1 {
        f2();
    } else {
        f3();
    }
    0
}

unsafe fn main_0() -> i32 {
    let id1 = thread::spawn(|| t_fun(0));
    let id2 = thread::spawn(|| t_fun(1));
    let id3 = thread::spawn(|| t_fun(2));

    id1.join().unwrap();
    id2.join().unwrap();
    id3.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            S1.lock().unwrap().n1,
            S2.lock().unwrap().n1,
            S3.lock().unwrap().n1,
        );
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}