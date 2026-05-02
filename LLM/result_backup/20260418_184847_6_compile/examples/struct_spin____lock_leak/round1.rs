use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Ss {
    pub n1: i32,
    pub m1: Arc<Mutex<i32>>,
    pub n2: i32,
    pub m2: Arc<Mutex<i32>>,
    pub n3: i32,
    pub m3: Arc<Mutex<i32>>,
}

#[no_mangle]
pub extern "C" fn f1(s: *mut Ss) {
    unsafe {
        let s = &mut *s;
        *s.m1.lock().unwrap() += 1;
        *s.m2.lock().unwrap() += 1;
        *s.m3.lock().unwrap() += 1;
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut Ss) -> *mut Ss {
    f1(arg);
    arg
}

unsafe fn main_0() -> i32 {
    let s = Arc::new(Ss {
        n1: 1,
        m1: Arc::new(Mutex::new(0)),
        n2: 2,
        m2: Arc::new(Mutex::new(0)),
        n3: 3,
        m3: Arc::new(Mutex::new(0)),
    });

    let s_clone1 = Arc::clone(&s);
    let s_clone2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone1) as *mut Ss);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone2) as *mut Ss);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}