use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;
use std::sync::OnceLock;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static S1: OnceLock<Arc<Mutex<Ss>>> = OnceLock::new();
static S2: OnceLock<Arc<Mutex<Ss>>> = OnceLock::new();

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s1 = S1.get().unwrap().lock().unwrap();
    f1(&mut *s1 as *mut Ss);
    drop(s1);

    let s2 = S2.get().unwrap().lock().unwrap();
    f1(&mut *s2 as *mut Ss);
    drop(s2);

    let s3 = Arc::from_raw(arg as *const Mutex<Ss>);
    let mut s3 = s3.lock().unwrap();
    f1(&mut *s3 as *mut Ss);
    drop(s3);

    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s3 = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));

    let s3_clone = Arc::clone(&s3);
    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    let s3_clone = Arc::clone(&s3);
    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s3 = Arc::try_unwrap(s3).unwrap();
    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        S1.get().unwrap().lock().unwrap().n,
        S2.get().unwrap().lock().unwrap().n,
        s3.lock().unwrap().n,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}