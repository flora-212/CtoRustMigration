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

unsafe extern "C" fn f1(s: &Arc<Ss>) {
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    f1(&S1);
    f1(&S2);
    let s3 = arg as *mut Ss;
    f1(&Arc::from_raw(s3));
    ptr::null_mut()
}

fn main_0() -> i32 {
    let s3 = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s3_clone = Arc::clone(&s3);

    let id1 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s3_clone)) };
    });

    let id2 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(Arc::clone(&s3))) };
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{} {} {}", S1.n, S2.n, s3.n);

    0
}

fn main() {
    std::process::exit(main_0());
}