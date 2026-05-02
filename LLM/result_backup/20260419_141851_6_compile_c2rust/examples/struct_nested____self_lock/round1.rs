use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

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

static mut S: Ss = Ss {
    s: Ss0 { n1: 0, n2: 1 },
    m: Mutex::new(()),
};

unsafe extern "C" fn f1() {
    let _guard = S.m.lock().unwrap();
    S.s.n1 += 1;
    S.s.n2 += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let _guard = S.m.lock().unwrap();
    f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s = Arc::new(S);
    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = s.clone();
        let handle = thread::spawn(move || {
            unsafe {
                t_fun(ptr::null_mut());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        libc::printf(
            b"%d %d\n\0".as_ptr(),
            S.s.n1,
            S.s.n2,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}