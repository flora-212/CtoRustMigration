use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

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

static S1: Arc<Ss1> = Arc::new(Ss1 {
    n1: 0,
    n2: 1,
    m1: Mutex::new(()),
});

static S2: Arc<Ss2> = Arc::new(Ss2 {
    n1: 2,
    n3: 3,
    m2: Mutex::new(()),
});

unsafe extern "C" fn f1() {
    let x = S1.n2 + S2.n3;
    let _guard1 = S1.m1.lock().unwrap();
    let _guard2 = S2.m2.lock().unwrap();
    S1.n1 += x;
    S2.n1 += x;
}

unsafe extern "C" fn f2() {
    let x = S1.n2 + S2.n3;
    let _guard2 = S2.m2.lock().unwrap();
    let _guard1 = S1.m1.lock().unwrap();
    S1.n1 += x;
    S2.n1 += x;
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
    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);

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
            S1.n1,
            S1.n2,
        );
        libc::printf(
            b"%d %d\n\0".as_ptr(),
            S2.n1,
            S2.n3,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}