use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct St {
    n1: i32,
    num_mutex: Mutex<()>,
}

static mut S1: Option<Arc<St>> = None;
static mut S2: Option<Arc<St>> = None;
static mut S3: Option<Arc<St>> = None;

static INIT: Once = Once::new();

fn init_globals() {
    unsafe {
        S1 = Some(Arc::new(St {
            n1: 0,
            num_mutex: Mutex::new(()),
        }));
        S2 = Some(Arc::new(St {
            n1: 1,
            num_mutex: Mutex::new(()),
        }));
        S3 = Some(Arc::new(St {
            n1: 2,
            num_mutex: Mutex::new(()),
        }));
    }
}

unsafe extern "C" fn f(s: *mut St) {
    let s = &mut *s;
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

unsafe extern "C" fn f1() {
    let s1 = S1.as_ref().unwrap().clone();
    let s2 = S2.as_ref().unwrap().clone();
    let s3 = S3.as_ref().unwrap().clone();

    f(&mut *s1.as_ptr());
    f(&mut *s2.as_ptr());
    f(&mut *s3.as_ptr());
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s1 = S1.as_ref().unwrap().clone();
    let s2 = S2.as_ref().unwrap().clone();
    let s3 = S3.as_ref().unwrap().clone();

    let handle1 = thread::spawn(move || {
        let _guard = s1.num_mutex.lock().unwrap();
        f1();
    });

    let handle2 = thread::spawn(move || {
        let _guard = s2.num_mutex.lock().unwrap();
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        S1.as_ref().unwrap().n1,
        S2.as_ref().unwrap().n1,
        S3.as_ref().unwrap().n1,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}