use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

lazy_static::lazy_static! {
    static ref S1: Arc<Mutex<Ss>> = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));
}

unsafe extern "C" fn f1(s: &Arc<Mutex<Ss>>) {
    let _guard = s.lock().unwrap();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s1 = S1.clone();
    let s = &*(arg as *const Arc<Mutex<Ss>>);
    f1(&s1);
    f1(s);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let s = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));

    let id1 = thread::spawn({
        let s = s.clone();
        move || {
            unsafe { t_fun(&s as *const _ as *mut libc::c_void) };
        }
    });

    let id2 = thread::spawn({
        let s = s.clone();
        move || {
            unsafe { t_fun(&s as *const _ as *mut libc::c_void) };
        }
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s = s.lock().unwrap();
    let s1 = S1.lock().unwrap();
    println!("{} {}", s.n, s1.n);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}