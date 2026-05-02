use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;

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
    let s1 = S1.clone();
    let s2 = S2.clone();
    let s3 = S3.clone();
    f(&s1);
    f(&s2);
    f(&s3);
}

unsafe extern "C" fn t_fun(_: *mut c_void) -> *mut c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s1 = S1.clone();
    let s2 = S2.clone();
    let s3 = S3.clone();

    let handle1 = thread::spawn(move || {
        f1();
    });

    let handle2 = thread::spawn(move || {
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            S1.n1,
            S2.n1,
            S3.n1,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}