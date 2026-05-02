use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

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

unsafe extern "C" fn h(u: *mut St) {
    let st = &mut *u;
    let _guard = st.num_mutex.lock().unwrap();
    st.n1 += 1;
}

unsafe extern "C" fn g(t: *mut St) {
    let st = &mut *t;
    st.n1 += 1;
    h(t);
}

unsafe extern "C" fn f(s: *mut St) {
    let st = &mut *s;
    let _guard = st.num_mutex.lock().unwrap();
    st.n1 += 1;
    g(s);
}

unsafe extern "C" fn f1() {
    f(&mut *S1.as_ptr());
    f(&mut *S2.as_ptr());
    f(&mut *S3.as_ptr());
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s1 = S1.clone();
    let s2 = S2.clone();
    let s3 = S3.clone();

    let handle1 = thread::spawn(move || {
        let _ = t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        let _ = t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        S1.n1,
        S2.n1,
        S3.n1,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}