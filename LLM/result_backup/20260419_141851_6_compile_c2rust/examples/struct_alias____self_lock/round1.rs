use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

#[no_mangle]
pub static mut S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static mut S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub static mut S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
});

#[no_mangle]
pub unsafe extern "C" fn h(u: *mut St) {
    let u = &mut *u;
    let _guard = u.num_mutex.lock().unwrap();
    u.n1 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn g(t: *mut St) {
    let t = &mut *t;
    t.n1 += 1;
    h(t);
}

#[no_mangle]
pub unsafe extern "C" fn f(s: *mut St) {
    let s = &mut *s;
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    f(&mut *S1.clone().as_ptr());
    f(&mut *S2.clone().as_ptr());
    f(&mut *S3.clone().as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );

    libc::pthread_create(
        &mut id2,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );

    libc::pthread_join(id1, ptr::null_mut());
    libc::pthread_join(id2, ptr::null_mut());

    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        S1.n1,
        S2.n1,
        S3.n1,
    );

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}