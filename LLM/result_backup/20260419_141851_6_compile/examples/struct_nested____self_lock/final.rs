use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

#[derive(Debug)]
struct Ss0 {
    n1: i32,
    n2: i32,
}

#[derive(Debug)]
struct Ss {
    s: Ss0,
    m: Mutex<()>,
}

lazy_static::lazy_static! {
    static ref S: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
        s: Ss0 { n1: 0, n2: 1 },
        m: Mutex::new(()),
    }));
}

unsafe extern "C" fn f1(s: &mut Ss) {
    let _guard = s.m.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut c_void) -> *mut c_void {
    let mut s = S.lock().unwrap();
    f1(&mut *s);
    ptr::null_mut()
}

fn main_0() -> i32 {
    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = Arc::clone(&S);
        let handle = thread::spawn(move || {
            unsafe {
                t_fun(ptr::null_mut());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let s = S.lock().unwrap();
    let n1 = s.s.n1;
    let n2 = s.s.n2;

    let c_string = CString::new(format!("{} {}\n", n1, n2)).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
