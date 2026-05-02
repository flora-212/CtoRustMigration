use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::CString;

static INIT: Once = Once::new();

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<i32>,
}

#[no_mangle]
pub static S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(0),
});

#[no_mangle]
pub static S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(0),
});

#[no_mangle]
pub static S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(0),
});

#[no_mangle]
pub extern "C" fn h(u: *const St) {
    unsafe {
        let u = &*u;
        let mut n1 = u.num_mutex.lock().unwrap();
        *n1 += 1;
    }
}

#[no_mangle]
pub extern "C" fn g(t: *const St) {
    unsafe {
        let t = &*t;
        let mut n1 = t.num_mutex.lock().unwrap();
        *n1 += 1;
    }
    h(t);
}

#[no_mangle]
pub extern "C" fn f(s: *const St) {
    unsafe {
        let s = &*s;
        let mut n1 = s.num_mutex.lock().unwrap();
        *n1 += 1;
    }
    g(s);
}

#[no_mangle]
pub extern "C" fn f1() {
    f(&S1);
    f(&S2);
    f(&S3);
}

#[no_mangle]
pub extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let id1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });
    let id2 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    id1.join().unwrap();
    id2.join().unwrap();

    unsafe {
        libc::printf(
            CString::new("%d %d %d\n").unwrap().as_ptr(),
            S1.n1,
            S2.n1,
            S3.n1,
        );
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}