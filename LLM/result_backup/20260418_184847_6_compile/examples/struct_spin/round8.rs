use std::sync::{Arc, Mutex, RwLock};
use std::thread;

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
pub extern "C" fn t_fun(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let s = arg as *mut Ss;
        f1(s);
    }
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    let s = Ss {
        n1: 1,
        m1: Arc::new(Mutex::new(0)),
        n2: 2,
        m2: Arc::new(RwLock::new(0)),
        n3: 3,
        m3: Arc::new(Mutex::new(0)),
    };

    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = Arc::clone(&s);
        let handle = thread::spawn(move || {
            f1(Arc::as_ptr(&s_clone) as *mut Ss);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

pub fn main() {
    std::process::exit(main_0() as i32);
}