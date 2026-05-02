use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<i32>,
}

static LOCK: Mutex<()> = Mutex::new(());

static mut X: Option<Arc<Ss>> = None;

unsafe extern "C" fn f1() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(0),
    });
    let mut s_ref = s.clone();
    let mut m = s_ref.m.lock().unwrap();
    *m = 456;
    unsafe { X = Some(s); }
}

unsafe extern "C" fn f2() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(0),
    });
    let mut s_ref = s.clone();
    let _guard = LOCK.lock().unwrap();
    let mut m = s_ref.m.lock().unwrap();
    *m = 789;
    unsafe { X = Some(s); }
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

    if let Some(x) = unsafe { X.take() } {
        println!("{}", x.n);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}