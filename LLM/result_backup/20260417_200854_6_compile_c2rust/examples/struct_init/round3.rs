use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

lazy_static::lazy_static! {
    static ref S1: Arc<Ss> = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
    static ref S2: Arc<Ss> = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
}

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    f1(&mut **S1 as *mut Ss);
    f1(&mut **S2 as *mut Ss);
    f1(arg as *mut Ss);
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s3 = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s3_clone = Arc::clone(&s3);

    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut std::ffi::c_void);
    });

    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(Arc::clone(&s3)) as *mut std::ffi::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{} {} {}", S1.n, S2.n, s3.n);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}