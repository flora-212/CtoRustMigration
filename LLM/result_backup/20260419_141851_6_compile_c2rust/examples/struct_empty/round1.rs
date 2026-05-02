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

unsafe extern "C" fn f1(s: &mut Ss) {
    let _guard = s.m.lock().unwrap();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s1_ptr = &mut S1 as *mut Ss;
    f1(&mut *s1_ptr);
    let arg_s = arg as *mut Ss;
    f1(&mut *arg_s);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Box::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);

    let id1 = thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    let id2 = thread::spawn(move || {
        t_fun(s_ptr as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s = Box::from_raw(s_ptr);
    libc::printf(
        b"%d %d\n\0".as_ptr(),
        s.n,
        S1.n,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}