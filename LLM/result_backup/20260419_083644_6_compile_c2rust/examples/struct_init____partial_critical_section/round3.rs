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
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s3 = arg as *mut Ss;
    f1(&mut *S1.lock().unwrap());
    f1(&mut *S2.lock().unwrap());
    f1(s3);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s3 = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });

    let s3_clone = Arc::clone(&s3);
    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    let s3_clone = Arc::clone(&s3);
    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{} {} {}", S1.lock().unwrap().n, S2.lock().unwrap().n, s3.lock().unwrap().n);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}