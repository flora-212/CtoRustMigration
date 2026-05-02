use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static mut S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));
static mut S2: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));

unsafe extern "C" fn f1(s: *mut Ss) {
    if let Some(s) = s.as_mut() {
        s.n += 1;
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s1 = S1.clone();
    let s2 = S2.clone();
    let s3 = arg as *mut Ss;

    f1(&mut *s1.lock().unwrap());
    f1(&mut *s2.lock().unwrap());
    f1(s3);

    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s3 = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));

    let id1 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s3.clone()) as *mut libc::c_void) };
    });

    let id2 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s3.clone()) as *mut libc::c_void) };
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let s3 = s3.lock().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            s1.n,
            s2.n,
            s3.n,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}