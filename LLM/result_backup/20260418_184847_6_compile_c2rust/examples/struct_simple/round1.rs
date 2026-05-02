use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<()>,
    m2: Mutex<()>,
}

static mut S: Arc<SharedData> = Arc::new(SharedData {
    n1: 0,
    n2: 1,
    n3: 2,
    n4: 3,
    m1: Mutex::new(()),
    m2: Mutex::new(()),
});

unsafe extern "C" fn f1() {
    let x = S.n4;
    let _guard1 = S.m1.lock().unwrap();
    S.n1 += x;
    S.n2 += x;
    let _guard2 = S.m2.lock().unwrap();
    S.n3 += x;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Arc::clone(&S);
    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });
    let s = Arc::clone(&S);
    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(
        b"%d %d %d %d\n\0".as_ptr() as *const libc::c_char,
        S.n1,
        S.n2,
        S.n3,
        S.n4,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}