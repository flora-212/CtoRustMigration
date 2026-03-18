use std::sync::{Arc, Mutex};
use std::thread;

pub struct ss {
    pub n: i32,
    pub m: Arc<Mutex<()>>,
}

static mut s1: ss = ss {
    n: 0,
    m: Arc::new(Mutex::new(())),
};

unsafe extern "C" fn f1(s: &ss) {
    let _guard = s.m.lock().unwrap();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = arg as *mut ss;
    f1(&*s);
    f1(&s1);
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Box::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });
    let s_clone = s.clone();

    let handle1 = thread::spawn(move || {
        t_fun(Box::into_raw(s) as *mut libc::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Box::into_raw(s_clone) as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
