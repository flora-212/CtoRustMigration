use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct Ss1 {
    n1: i32,
    n2: i32,
    m1: Mutex<i32>,
}

#[derive(Debug)]
struct Ss2 {
    n1: i32,
    n3: i32,
    m2: Mutex<i32>,
}

static mut S1: Option<Arc<Ss1>> = None;
static mut S2: Option<Arc<Ss2>> = None;
static INIT: Once = Once::new();

fn init_globals() {
    unsafe {
        S1 = Some(Arc::new(Ss1 {
            n1: 0,
            n2: 1,
            m1: Mutex::new(0),
        }));
        S2 = Some(Arc::new(Ss2 {
            n1: 2,
            n3: 3,
            m2: Mutex::new(0),
        }));
    }
}

unsafe extern "C" fn f1() {
    INIT.call_once(init_globals);
    let x = S1.as_ref().unwrap().n2 + S2.as_ref().unwrap().n3;
    let mut guard1 = S1.as_ref().unwrap().m1.lock().unwrap();
    let mut guard2 = S2.as_ref().unwrap().m2.lock().unwrap();
    *guard1 += x;
    *guard2 += x;
}

unsafe extern "C" fn f2() {
    INIT.call_once(init_globals);
    let x = S1.as_ref().unwrap().n2 + S2.as_ref().unwrap().n3;
    let mut guard2 = S2.as_ref().unwrap().m2.lock().unwrap();
    let mut guard1 = S1.as_ref().unwrap().m1.lock().unwrap();
    *guard1 += x;
    *guard2 += x;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    if arg as libc::c_long == 0 {
        f1();
    } else {
        f2();
    }
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s1 = S1.as_ref().unwrap().clone();
    let s2 = S2.as_ref().unwrap().clone();

    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(1 as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d\n\0".as_ptr(),
            S1.as_ref().unwrap().n1,
            S1.as_ref().unwrap().n2,
        );
        libc::printf(
            b"%d %d\n\0".as_ptr(),
            S2.as_ref().unwrap().n1,
            S2.as_ref().unwrap().n3,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}