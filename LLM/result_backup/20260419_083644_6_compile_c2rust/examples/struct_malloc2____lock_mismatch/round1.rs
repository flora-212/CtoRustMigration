use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static LOCK: Mutex<()> = Mutex::new(());

static mut X: Option<Arc<Ss>> = None;

unsafe extern "C" fn f1() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(()),
    });
    let mut s_clone = s.clone();
    X = Some(s);

    let _guard = s_clone.m.lock().unwrap();
    s_clone.n = 456;
}

unsafe extern "C" fn f2() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(()),
    });
    let mut s_clone = s.clone();
    X = Some(s);

    let _guard = LOCK.lock().unwrap();
    s_clone.n = 789;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });
    let handle2 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    if let Some(x) = X {
        println!("{}", x.n);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}