use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::CString;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static mut S1: Option<Arc<Ss>> = None;
static mut S2: Option<Arc<Ss>> = None;
static INIT: Once = Once::new();

fn init_globals() {
    unsafe {
        S1 = Some(Arc::new(Ss {
            n: 0,
            m: Mutex::new(()),
        }));
        S2 = Some(Arc::new(Ss {
            n: 0,
            m: Mutex::new(()),
        }));
    }
}

unsafe extern "C" fn f1(s: *mut Ss) {
    if let Some(s) = s.as_mut() {
        s.n += 1;
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    INIT.call_once(init_globals);
    f1(&mut *S1.as_ref().unwrap().lock().unwrap());
    f1(&mut *S2.as_ref().unwrap().lock().unwrap());
    if let Some(s) = arg as *mut Ss as *mut Option<Ss>.as_mut() {
        if let Some(s) = s.take() {
            f1(&mut s);
        }
    }
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s3 = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });

    let s3_clone = Arc::clone(&s3);

    let handle1 = thread::spawn(move || {
        unsafe {
            t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
        }
    });

    let handle2 = thread::spawn(move || {
        unsafe {
            t_fun(Arc::into_raw(Arc::clone(&s3)) as *mut libc::c_void);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(
            CString::new("%d %d %d\n").unwrap().as_ptr(),
            S1.as_ref().unwrap().lock().unwrap().n,
            S2.as_ref().unwrap().lock().unwrap().n,
            s3.lock().unwrap().n,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}