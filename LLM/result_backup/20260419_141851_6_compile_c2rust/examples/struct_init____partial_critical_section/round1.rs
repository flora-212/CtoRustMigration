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

static mut S2: Ss = Ss {
    n: 0,
    m: Mutex::new(()),
};

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(&mut S1 as *mut Ss);
    f1(&mut S2 as *mut Ss);
    f1(arg as *mut Ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s3 = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));

    let s3_clone = Arc::clone(&s3);
    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    let s3_clone = Arc::clone(&s3);
    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s3 = Arc::try_unwrap(s3).unwrap();
    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        S1.n,
        S2.n,
        s3.n,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}