use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Mutex<()>,
}

#[no_mangle]
pub static S1: OnceLock<Arc<St>> = OnceLock::new();

#[no_mangle]
pub static S2: OnceLock<Arc<St>> = OnceLock::new();

#[no_mangle]
pub static S3: OnceLock<Arc<St>> = OnceLock::new();

fn init_s1() -> Arc<St> {
    Arc::new(St {
        n1: 0,
        num_mutex: Mutex::new(()),
    })
}

fn init_s2() -> Arc<St> {
    Arc::new(St {
        n1: 1,
        num_mutex: Mutex::new(()),
    })
}

fn init_s3() -> Arc<St> {
    Arc::new(St {
        n1: 2,
        num_mutex: Mutex::new(()),
    })
}

#[no_mangle]
pub fn f(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

#[no_mangle]
pub fn f1() {
    f(S1.get().unwrap());
    f(S2.get().unwrap());
    f(S3.get().unwrap());
}

#[no_mangle]
pub fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    S1.get_or_init(init_s1);
    S2.get_or_init(init_s2);
    S3.get_or_init(init_s3);

    let s1 = Arc::clone(S1.get().unwrap());
    let s2 = Arc::clone(S2.get().unwrap());
    let s3 = Arc::clone(S3.get().unwrap());

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            S1.get().unwrap().n1,
            S2.get().unwrap().n1,
            S3.get().unwrap().n1,
        );
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}