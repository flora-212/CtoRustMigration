use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Debug)]
pub struct Ss {
    n1: i32,
    m1: Mutex<i32>,
    n2: i32,
    m2: RwLock<i32>,
    n3: i32,
    m3: Mutex<i32>,
}

#[no_mangle]
pub extern "C" fn f1(s: &mut Ss) {
    *s.m1.lock().unwrap() += 1;
    *s.m2.write().unwrap() += 1;
    *s.m3.lock().unwrap() += 1;
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut Ss) -> *mut Ss {
    unsafe {
        f1(&mut *arg);
    }
    arg
}

unsafe fn main_0() -> i32 {
    let mut s = Ss {
        n1: 0,
        m1: Mutex::new(0),
        n2: 0,
        m2: RwLock::new(0),
        n3: 0,
        m3: Mutex::new(0),
    };

    s.n1 = 1;
    s.n2 = 2;
    s.n3 = 3;

    let s = Arc::new(s);
    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::as_ptr(&s1) as *mut Ss);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::as_ptr(&s2) as *mut Ss);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
