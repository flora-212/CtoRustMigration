use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

lazy_static! {
    static ref S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));
    static ref S2: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));
}

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(&mut S1.lock().unwrap());
    f1(&mut S2.lock().unwrap());
    f1(arg as *mut Ss);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s3 = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));

    let s3_clone = Arc::clone(&s3);
    let handle1 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void) };
    });

    let s3_clone = Arc::clone(&s3);
    let handle2 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr(),
            S1.lock().unwrap().n,
            S2.lock().unwrap().n,
            (*Arc::as_ptr(&s3)).lock().unwrap().n,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}