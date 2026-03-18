use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
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
    let s = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });

    let s_clone = Arc::clone(&s);
    let s_ptr = Arc::into_raw(s_clone);

    let t1 = thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    let t2 = thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let _ = Arc::from_raw(s_ptr);

    0 as libc::c_int
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
