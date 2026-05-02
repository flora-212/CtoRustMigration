use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<i32>,
}

static S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
    n: 0,
    m: Mutex::new(0),
}));

static S2: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
    n: 0,
    m: Mutex::new(0),
}));

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    let mut guard = s.m.lock().unwrap();
    *guard += 1;
}

unsafe extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let s1 = &mut **S1.lock().unwrap() as *mut Ss;
    let s2 = &mut **S2.lock().unwrap() as *mut Ss;
    f1(s1);
    f1(s2);
    f1(arg as *mut Ss);
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s3 = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(0),
    }));
    let s3_clone = Arc::clone(&s3);

    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut std::ffi::c_void);
    });

    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(Arc::clone(&s3)) as *mut std::ffi::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let s3 = s3.lock().unwrap();

    println!("{} {} {}", s1.n, s2.n, s3.n);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}