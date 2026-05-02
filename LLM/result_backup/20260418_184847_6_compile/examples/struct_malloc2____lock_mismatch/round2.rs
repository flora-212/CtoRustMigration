use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static LOCK: Mutex<()> = Mutex::new(());

static mut X: Option<Arc<Ss>> = None;

unsafe extern "C" fn f1() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(()),
    });
    let mut s_ref = s.clone();

    unsafe {
        X = Some(s);
    }

    let mut s_ref2 = s_ref.clone();
    let _guard = s_ref2.m.lock().unwrap();
    s_ref2.n = 456;
}

unsafe extern "C" fn f2() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(()),
    });
    let mut s_ref = s.clone();

    unsafe {
        X = Some(s);
    }

    let _guard = LOCK.lock().unwrap();
    s_ref.n = 789;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    if let Some(x) = unsafe { X.take() } {
        let n = x.n;
        let c_string = CString::new(format!("{}\n", n)).unwrap();
        unsafe {
            libc::printf(c_string.as_ptr());
        }
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}