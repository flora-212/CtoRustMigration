use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

#[no_mangle]
pub static S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub fn f(s: &Arc<St>, t: &Arc<St>) {
    let _guard_s = s.num_mutex.lock().unwrap();
    let _guard_t = t.num_mutex.lock().unwrap();
    s.n1 = t.n1 + 1;
    t.n1 = s.n1 + 1;
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
pub fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    if arg as isize == 0 {
        f1();
    } else if arg as isize == 1 {
        f2();
    } else {
        f3();
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);
    let s3 = Arc::clone(&S3);

    let handle1 = thread::spawn(move || {
        t_fun(2 as *mut std::ffi::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(1 as *mut std::ffi::c_void);
    });

    let handle3 = thread::spawn(move || {
        t_fun(3 as *mut std::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    println!("{} {} {}", S1.n1, S2.n1, S3.n1);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}