use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::sync::OnceLock;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static S1: OnceLock<Arc<Mutex<Ss>>> = OnceLock::new();
static S2: OnceLock<Arc<Mutex<Ss>>> = OnceLock::new();

unsafe extern "C" fn f1(s: *mut Ss) {
    (*s).n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s1 = &mut *S1.get().unwrap().lock().unwrap();
    let s2 = &mut *S2.get().unwrap().lock().unwrap();
    let s3 = &mut *(arg as *mut Ss);
    f1(s1);
    f1(s2);
    f1(s3);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    S1.set(Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }))).unwrap();
    S2.set(Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }))).unwrap();

    let s3 = Arc::new(Mutex::new(Ss { n: 0, m: Mutex::new(()) }));
    let s3_clone = Arc::clone(&s3);

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(Arc::into_raw(Arc::clone(&s3)) as *mut libc::c_void) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = &mut *S1.get().unwrap().lock().unwrap();
    let s2 = &mut *S2.get().unwrap().lock().unwrap();
    let s3 = &mut *s3.lock().unwrap();

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