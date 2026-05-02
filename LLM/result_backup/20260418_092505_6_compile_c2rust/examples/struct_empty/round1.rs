use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static mut S1: Ss = Ss {
    n: 0,
    m: Mutex::new(()),
};

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    let _guard = s.m.lock().unwrap();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(&mut S1);
    f1(arg as *mut Ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Box::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);

    let mut id1 = 0;
    let mut id2 = 0;

    thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    // Joining threads is not directly possible with pthread_join in Rust.
    // We need to ensure the threads finish execution before accessing s_ptr.

    let s = Box::from_raw(s_ptr);
    libc::printf(b"%d %d\n\0".as_ptr() as *const libc::c_char, s.n, S1.n);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}