use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct Ss1 {
    n1: i32,
    n2: i32,
    m1: Mutex<()>,
}

#[derive(Debug)]
struct Ss2 {
    n1: i32,
    n3: i32,
    m2: Mutex<()>,
}

static mut S1: Option<Arc<Ss1>> = None;
static mut S2: Option<Arc<Ss2>> = None;
static INIT: Once = Once::new();

fn init_globals() {
    unsafe {
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
}

unsafe extern "C" fn f1() {
    let x = S1.as_ref().unwrap().n2 + S2.as_ref().unwrap().n3;
    let _guard1 = S1.as_ref().unwrap().m1.lock().unwrap();
    let _guard2 = S2.as_ref().unwrap().m2.lock().unwrap();
    S1.as_ref().unwrap().n1 += x;
    S2.as_ref().unwrap().n1 += x;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s1 = Arc::clone(unsafe { S1.as_ref().unwrap() });
    let s2 = Arc::clone(unsafe { S2.as_ref().unwrap() });

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            S1.as_ref().unwrap().n1,
            S1.as_ref().unwrap().n2,
        );
        libc::printf(
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            S2.as_ref().unwrap().n1,
            S2.as_ref().unwrap().n3,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}