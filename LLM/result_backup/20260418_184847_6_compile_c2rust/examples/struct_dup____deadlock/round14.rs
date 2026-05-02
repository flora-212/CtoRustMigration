use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::Once;

#[derive(Debug)]
struct Ss1 {
    n1: c_int,
    n2: c_int,
    m1: Mutex<()>,
}

#[derive(Debug)]
struct Ss2 {
    n1: c_int,
    n3: c_int,
    m2: Mutex<()>,
}

static mut S1: Option<Arc<Mutex<Ss1>>> = None;
static mut S2: Option<Arc<Mutex<Ss2>>> = None;
static INIT: Once = Once::new();

fn initialize_globals() {
    unsafe {
        INIT.call_once(|| {
            S1 = Some(Arc::new(Mutex::new(Ss1 {
                n1: 0,
                n2: 1,
                m1: Mutex::new(()),
            })));
            S2 = Some(Arc::new(Mutex::new(Ss2 {
                n1: 2,
                n3: 3,
                m2: Mutex::new(()),
            })));
        });
    }
}

unsafe extern "C" fn f1(s1: &Arc<Mutex<Ss1>>, s2: &Arc<Mutex<Ss2>>) {
    let x = {
        let s1 = s1.lock().unwrap();
        let s2 = s2.lock().unwrap();
        s1.n2 + s2.n3
    };
    let _guard1 = s1.lock().unwrap();
    let _guard2 = s2.lock().unwrap();
    s1.lock().unwrap().n1 += x;
    s2.lock().unwrap().n1 += x;
}

unsafe extern "C" fn f2(s1: &Arc<Mutex<Ss1>>, s2: &Arc<Mutex<Ss2>>) {
    let x = {
        let s1 = s1.lock().unwrap();
        let s2 = s2.lock().unwrap();
        s1.n2 + s2.n3
    };
    let _guard2 = s2.lock().unwrap();
    let _guard1 = s1.lock().unwrap();
    s1.lock().unwrap().n1 += x;
    s2.lock().unwrap().n1 += x;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void, s1: &Arc<Mutex<Ss1>>, s2: &Arc<Mutex<Ss2>>) -> *mut libc::c_void {
    if arg as libc::c_long == 0 {
        f1(s1, s2);
    } else {
        f2(s1, s2);
    }
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    initialize_globals();

    let s1 = Arc::clone(&S1.as_ref().unwrap());
    let s2 = Arc::clone(&S2.as_ref().unwrap());

    let handle1 = thread::spawn(move || {
        t_fun(std::ptr::null_mut(), &s1, &s2);
    });

    let handle2 = thread::spawn(move || {
        t_fun(1 as *mut libc::c_void, &s1, &s2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s1 = Arc::try_unwrap(S1.take().unwrap()).unwrap().into_inner().unwrap();
    let s2 = Arc::try_unwrap(S2.take().unwrap()).unwrap().into_inner().unwrap();

    let c_str1 = CString::new(format!("{} {}\n", s1.n1, s1.n2)).unwrap();
    libc::printf(c_str1.as_ptr());

    let c_str2 = CString::new(format!("{} {}\n", s2.n1, s2.n3)).unwrap();
    libc::printf(c_str2.as_ptr());

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}