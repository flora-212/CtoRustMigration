use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use libc;
use std::sync::Once;

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

static mut S: Option<Arc<Ss>> = None;
static INIT: Once = Once::new();

fn get_s() -> Arc<Ss> {
    INIT.call_once(|| {
        unsafe {
            S = Some(Arc::new(Ss {
                s: Ss0 { n1: 0, n2: 1 },
                m: Mutex::new(()),
            }));
        }
    });
    unsafe { S.clone().unwrap() }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let s = get_s();
    let _guard = s.m.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = get_s();
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(move || {
            f1();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let s = get_s();
    let _guard = s.m.lock().unwrap();
    let c_string = CString::new(format!("{} {}\n", s.s.n1, s.s.n2)).unwrap();
    libc::printf(c_string.as_ptr());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}