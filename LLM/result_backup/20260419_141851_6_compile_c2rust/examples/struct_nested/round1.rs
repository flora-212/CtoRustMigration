use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Ss0 {
    n1: i32,
    n2: i32,
}

#[derive(Debug)]
struct Ss {
    s: Ss0,
    m: Mutex<()>,
}

static mut S: Arc<Ss> = Arc::new(Ss {
    s: Ss0 { n1: 0, n2: 1 },
    m: Mutex::new(()),
});

unsafe extern "C" fn f1() {
    let s = S.clone();
    let _guard = s.m.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s = unsafe { S.clone() };

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
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            S.s.n1,
            S.s.n2,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}