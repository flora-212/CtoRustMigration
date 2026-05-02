use std::sync::{Arc, Mutex};
use std::thread;

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

unsafe extern "C" fn f(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

unsafe extern "C" fn f1() {
    f(&S1);
    f(&S2);
    f(&S3);
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> libc::c_int {
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