use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::Once;

static mut S1: Arc<Ss1> = Arc::new(Ss1 {
    n1: 0,
    n2: 1,
    m1: Mutex::new(()),
});

static mut S2: Arc<Ss2> = Arc::new(Ss2 {
    n1: 2,
    n3: 3,
    m2: Mutex::new(()),
});

static INIT: Once = Once::new();

fn init_globals() {
    unsafe {
        S1 = Arc::new(Ss1 {
            n1: 0,
            n2: 1,
            m1: Mutex::new(()),
        });
        S2 = Arc::new(Ss2 {
            n1: 2,
            n3: 3,
            m2: Mutex::new(()),
        });
    }
}

unsafe extern "C" fn f1() {
    let x = S1.n2 + S2.n3;
    let _guard1 = S1.m1.lock().unwrap();
    let _guard2 = S2.m2.lock().unwrap();
    S1.n1 += x;
    S2.n1 += x;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> c_int {
    INIT.call_once(init_globals);

    let s1 = unsafe { Arc::clone(&S1) };
    let s2 = unsafe { Arc::clone(&S2) };

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = unsafe { Arc::clone(&S1) };
    let s2 = unsafe { Arc::clone(&S2) };

    let c_str1 = CString::new(format!("{} {}\n", s1.n1, s1.n2)).unwrap();
    let c_str2 = CString::new(format!("{} {}\n", s2.n1, s2.n3)).unwrap();

    unsafe {
        libc::printf(c_str1.as_ptr());
        libc::printf(c_str2.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}