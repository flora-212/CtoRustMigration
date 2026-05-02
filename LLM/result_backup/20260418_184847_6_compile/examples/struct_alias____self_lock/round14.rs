use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::c_void;
use std::sync::OnceLock;

#[derive(Debug)]
struct St {
    n1: i32,
    num_mutex: Mutex<()>,
}

static S1: OnceLock<Arc<Mutex<St>>> = OnceLock::new();
static S2: OnceLock<Arc<Mutex<St>>> = OnceLock::new();
static S3: OnceLock<Arc<Mutex<St>>> = OnceLock::new();

fn get_s1() -> &'static Arc<Mutex<St>> {
    S1.get_or_init(|| Arc::new(Mutex::new(St {
        n1: 0,
        num_mutex: Mutex::new(()),
    })))
}

fn get_s2() -> &'static Arc<Mutex<St>> {
    S2.get_or_init(|| Arc::new(Mutex::new(St {
        n1: 1,
        num_mutex: Mutex::new(()),
    })))
}

fn get_s3() -> &'static Arc<Mutex<St>> {
    S3.get_or_init(|| Arc::new(Mutex::new(St {
        n1: 2,
        num_mutex: Mutex::new(()),
    })))
}

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
    let s1 = get_s1().lock().unwrap();
    let s2 = get_s2().lock().unwrap();
    let s3 = get_s3().lock().unwrap();

    f(&mut *s1);
    f(&mut *s2);
    f(&mut *s3);
}

unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let s1 = get_s1().clone();
    let s2 = get_s2().clone();
    let s3 = get_s3().clone();

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
        get_s1().lock().unwrap().n1,
        get_s2().lock().unwrap().n1,
        get_s3().lock().unwrap().n1,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}