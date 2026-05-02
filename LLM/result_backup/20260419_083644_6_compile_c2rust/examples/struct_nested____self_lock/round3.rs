use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;

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

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut s = S.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = Arc::clone(&S);
    let _guard = s.lock().unwrap();
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Arc::clone(&S);
    let handle1 = thread::spawn(move || {
        let _guard = s.lock().unwrap();
        f1();
    });

    let s = Arc::clone(&S);
    let handle2 = thread::spawn(move || {
        let _guard = s.lock().unwrap();
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.lock().unwrap();
    let message = CString::new(format!("{} {}\n", s.s.n1, s.s.n2)).unwrap();
    libc::printf(message.as_ptr() as *const c_char);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}