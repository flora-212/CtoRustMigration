use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;

#[derive(Debug)]
struct St {
    n1: i32,
    num_mutex: Mutex<()>,
}

static S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
});

static S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
});

static S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
});

unsafe extern "C" fn h(s: &mut St) {
    s.n1 += 1;
}

unsafe extern "C" fn g(s: &mut St) {
    s.n1 += 1;
    h(s);
}

unsafe extern "C" fn f(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(Arc::make_mut(s));
}

unsafe extern "C" fn f1() {
    let mut s1 = S1.clone();
    let mut s2 = S2.clone();
    let mut s3 = S3.clone();
    f(&s1);
    f(&s2);
    f(&s3);
}

unsafe extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> c_int {
    let s1 = S1.clone();
    let s2 = S2.clone();
    let s3 = S3.clone();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = S1.lock().unwrap();
    let s2 = S2.lock().unwrap();
    let s3 = S3.lock().unwrap();

    let message = format!("{} {} {}\n", s1.n1, s2.n1, s3.n1);
    let c_message = CString::new(message).unwrap();
    unsafe {
        libc::printf(c_message.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}