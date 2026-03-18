use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Copy, Clone)]
pub struct ss {
    pub n: i32,
    pub m: Arc<Mutex<()>>,
}

static mut s1: ss = ss {
    n: 0,
    m: Arc::new(Mutex::new(())),
};

unsafe extern "C" fn f1(s: &ss) {
    let _lock = s.m.lock().unwrap();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = arg as *mut ss;
    f1(&*s);
    f1(&s1);
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let s = Box::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });
    let s_ptr = Box::into_raw(s);

    let t1 = thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    let t2 = thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    libc::free(s_ptr as *mut libc::c_void);

    0 as libc::c_int
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}