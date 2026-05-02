use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

#[no_mangle]
static S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));

#[no_mangle]
static S2: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));

#[no_mangle]
unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    let mut lock = s.m.lock().unwrap();
    s.n += 1;
}

#[no_mangle]
unsafe extern "C" fn t_fun(arg: *mut Ss) -> *mut Ss {
    f1(&mut *S1.lock().unwrap());
    f1(&mut *S2.lock().unwrap());
    f1(arg);
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s3 = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));
    let s3_clone = Arc::clone(&s3);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut Ss);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(Arc::clone(&s3)) as *mut Ss);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let s3 = s3.lock().unwrap();

    println!("{} {} {}", s1.n, s2.n, s3.n);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}