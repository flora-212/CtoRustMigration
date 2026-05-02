use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::ptr;

#[derive(Debug)]
pub struct Ss {
    pub n1: i32,
    pub m1: Arc<Mutex<i32>>,
    pub n2: i32,
    pub m2: Arc<RwLock<i32>>,
    pub n3: i32,
    pub m3: Arc<Mutex<i32>>,
}

#[no_mangle]
pub extern "C" fn f1(s: *mut Ss) {
    unsafe {
        let s = &mut *s;
        *s.m1.lock().unwrap() += 1;
        *s.m2.write().unwrap() += 1;
        *s.m3.lock().unwrap() += 1;
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut Ss) -> *mut std::ffi::c_void {
    f1(arg);
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let mut s = Ss {
        n1: 1,
        m1: Arc::new(Mutex::new(0)),
        n2: 2,
        m2: Arc::new(RwLock::new(0)),
        n3: 3,
        m3: Arc::new(Mutex::new(0)),
    };

    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || {
        f1(&mut s1 as *mut Ss);
    });

    let handle2 = thread::spawn(move || {
        f1(&mut s2 as *mut Ss);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}