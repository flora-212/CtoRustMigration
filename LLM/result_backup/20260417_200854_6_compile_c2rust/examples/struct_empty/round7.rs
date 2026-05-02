use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
    n: 0,
    m: Mutex::new(()),
}));

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    let _guard = s.m.lock().unwrap();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s1 = S1.lock().unwrap();
    f1(&mut *s1 as *mut Ss);
    let s = &mut *(arg as *mut Ss);
    f1(s);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Box::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);

    let mut handles = vec![];

    for _ in 0..2 {
        let s_clone = Arc::new(Mutex::new(s_ptr));
        let handle = thread::spawn(move || {
            let s = Arc::clone(&s_clone);
            let s = unsafe { &mut *s.lock().unwrap() };
            t_fun(s as *mut Ss as *mut libc::c_void);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let s = unsafe { Box::from_raw(s_ptr) };
    libc::printf(
        b"%d %d\n\0".as_ptr() as *const libc::c_char,
        s.n,
        S1.lock().unwrap().n,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}