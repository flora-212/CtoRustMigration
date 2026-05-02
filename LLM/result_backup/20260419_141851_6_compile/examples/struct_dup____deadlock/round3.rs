use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::Once;

static mut S1: Option<Arc<Ss1>> = None;
static mut S2: Option<Arc<Ss2>> = None;

static INIT: Once = Once::new();

struct Ss1 {
    n1: c_int,
    n2: c_int,
    m1: Mutex<()>,
}

struct Ss2 {
    n1: c_int,
    n3: c_int,
    m2: Mutex<()>,
}

unsafe fn init_globals() {
    S1 = Some(Arc::new(Ss1 {
        n1: 0,
        n2: 1,
        m1: Mutex::new(()),
    }));
    S2 = Some(Arc::new(Ss2 {
        n1: 2,
        n3: 3,
        m2: Mutex::new(()),
    }));
}

unsafe extern "C" fn f1() {
    let x = (*S1.as_ref().unwrap()).n2 + (*S2.as_ref().unwrap()).n3;
    let _guard1 = (*S1.as_ref().unwrap()).m1.lock().unwrap();
    let _guard2 = (*S2.as_ref().unwrap()).m2.lock().unwrap();
    (*S1.as_ref().unwrap()).n1 += x;
    (*S2.as_ref().unwrap()).n1 += x;
}

unsafe extern "C" fn f2() {
    let x = (*S1.as_ref().unwrap()).n2 + (*S2.as_ref().unwrap()).n3;
    let _guard2 = (*S2.as_ref().unwrap()).m2.lock().unwrap();
    let _guard1 = (*S1.as_ref().unwrap()).m1.lock().unwrap();
    (*S1.as_ref().unwrap()).n1 += x;
    (*S2.as_ref().unwrap()).n1 += x;
}

unsafe extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    if arg as c_int == 0 {
        f1();
    } else {
        f2();
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    INIT.call_once(|| {
        init_globals();
    });

    let s1 = Arc::clone(S1.as_ref().unwrap());
    let s2 = Arc::clone(S2.as_ref().unwrap());

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(1 as *mut std::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = Arc::clone(S1.as_ref().unwrap());
    let s2 = Arc::clone(S2.as_ref().unwrap());

    let cstr1 = CString::new(format!("{} {}\n", s1.n1, s1.n2)).unwrap();
    let cstr2 = CString::new(format!("{} {}\n", s2.n1, s2.n3)).unwrap();

    unsafe {
        libc::printf(cstr1.as_ptr());
        libc::printf(cstr2.as_ptr());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}