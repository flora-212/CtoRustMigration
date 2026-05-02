use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;
use std::sync::OnceLock;

static mut S1: OnceLock<Arc<St>> = OnceLock::new();
static mut S2: OnceLock<Arc<St>> = OnceLock::new();
static mut S3: OnceLock<Arc<St>> = OnceLock::new();

static INIT: Once = Once::new();

struct St {
    n1: i32,
    num_mutex: Mutex<()>,
}

fn init_globals() {
    unsafe {
        S1.set(Arc::new(St {
            n1: 0,
            num_mutex: Mutex::new(()),
        })).unwrap();
        S2.set(Arc::new(St {
            n1: 1,
            num_mutex: Mutex::new(()),
        })).unwrap();
        S3.set(Arc::new(St {
            n1: 2,
            num_mutex: Mutex::new(()),
        })).unwrap();
    }
}

unsafe extern "C" fn f(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

unsafe extern "C" fn f1() {
    f(S1.get().unwrap());
    f(S2.get().unwrap());
    f(S3.get().unwrap());
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s1 = S1.get().unwrap().clone();
    let s2 = S2.get().unwrap().clone();
    let s3 = S3.get().unwrap().clone();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
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

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}