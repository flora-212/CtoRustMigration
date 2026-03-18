use std::sync::{Arc, Mutex};
use std::thread;

pub struct ss {
    pub n: i32,
    pub m: Arc<Mutex<()>>,
}

lazy_static::lazy_static! {
    static ref S1: Arc<ss> = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });
}

extern "C" fn f1(s: &ss) {
    let _guard = s.m.lock().unwrap();
}

extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = unsafe { &mut *(arg as *mut ss) };
    f1(&S1);
    f1(s);
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    let s = Arc::new(ss {
        n: 0,
        m: Arc::clone(&S1.m),
    });

    let s_clone = Arc::clone(&s);
    let handle1 = thread::spawn(move || {
        unsafe { t_fun(&mut *s_clone as *mut ss as *mut libc::c_void) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(&mut *s as *mut ss as *mut libc::c_void) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
