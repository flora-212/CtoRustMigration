use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

#[no_mangle]
pub static mut S1: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
}));

#[no_mangle]
pub static mut S2: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
}));

#[no_mangle]
pub static mut S3: Arc<Mutex<St>> = Arc::new(Mutex::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
}));

#[no_mangle]
pub unsafe extern "C" fn h(s: &mut St) {
    s.n1 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn g(s: &mut St) {
    s.n1 += 1;
    h(s);
}

#[no_mangle]
pub unsafe extern "C" fn f(s: &mut St) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut s1 = S1.lock().unwrap();
    let mut s2 = S2.lock().unwrap();
    let mut s3 = S3.lock().unwrap();
    f(&mut s1);
    f(&mut s2);
    f(&mut s3);
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );

    libc::pthread_join(id1, std::ptr::null_mut());
    libc::pthread_join(id2, std::ptr::null_mut());

    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        S1.lock().unwrap().n1,
        S2.lock().unwrap().n1,
        S3.lock().unwrap().n1,
    );

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}