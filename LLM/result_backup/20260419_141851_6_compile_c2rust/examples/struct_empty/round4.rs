use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

lazy_static::lazy_static! {
    static ref S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));
}

unsafe extern "C" fn f1(s: &mut Ss) {
    let _guard = s.m.lock().unwrap();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut s1 = S1.lock().unwrap();
    f1(&mut *s1);
    let arg_s = arg as *mut Ss;
    f1(&mut *arg_s);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));

    let s_clone = Arc::clone(&s);
    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone) as *mut libc::c_void);
    });

    let s_clone = Arc::clone(&s);
    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s_clone) as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s = Arc::try_unwrap(s).unwrap().into_inner().unwrap();
    libc::printf(
        b"%d %d\n\0".as_ptr(),
        s.n,
        S1.lock().unwrap().n,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}