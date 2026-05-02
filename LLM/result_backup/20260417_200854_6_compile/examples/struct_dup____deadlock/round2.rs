use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

static INIT: Once = Once::new();

#[derive(Debug)]
pub struct Ss1 {
    pub n1: i32,
    pub n2: i32,
    pub m1: Mutex<()>,
}

#[derive(Debug)]
pub struct Ss2 {
    pub n1: i32,
    pub n3: i32,
    pub m2: Mutex<()>,
}

#[no_mangle]
pub static mut S1: Arc<Ss1> = Arc::new(Ss1 {
    n1: 0,
    n2: 1,
    m1: Mutex::new(()),
});

#[no_mangle]
pub static mut S2: Arc<Ss2> = Arc::new(Ss2 {
    n1: 2,
    n3: 3,
    m2: Mutex::new(()),
});

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

#[no_mangle]
pub fn f1() {
    INIT.call_once(init_globals);

    let x = unsafe { S1.n2 + S2.n3 };
    let s1 = unsafe { S1.clone() };
    let s2 = unsafe { S2.clone() };

    let _guard1 = s1.m1.lock().unwrap();
    let _guard2 = s2.m2.lock().unwrap();

    s1.n1 += x;
    s2.n1 += x;
}

#[no_mangle]
pub fn f2() {
    INIT.call_once(init_globals);

    let x = unsafe { S1.n2 + S2.n3 };
    let s1 = unsafe { S1.clone() };
    let s2 = unsafe { S2.clone() };

    let _guard2 = s2.m2.lock().unwrap();
    let _guard1 = s1.m1.lock().unwrap();

    s1.n1 += x;
    s2.n1 += x;
}

#[no_mangle]
pub fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    INIT.call_once(init_globals);

    if arg as libc::c_long == 0 {
        f1();
    } else {
        f2();
    }
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s1 = unsafe { S1.clone() };
    let s2 = unsafe { S2.clone() };

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
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            S1.n1,
            S1.n2,
        );
        libc::printf(
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            S2.n1,
            S2.n3,
        );
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}