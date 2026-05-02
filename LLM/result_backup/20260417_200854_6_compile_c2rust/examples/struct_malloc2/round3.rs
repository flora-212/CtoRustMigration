use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static X: Arc<Mutex<Option<Arc<Ss>>>> = Arc::new(Mutex::new(None));

unsafe extern "C" fn f1() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(()),
    });
    let mut x = X.lock().unwrap();
    *x = Some(s.clone());
    drop(x);

    let mut guard = s.m.lock().unwrap();
    s.n = 456;
    drop(guard);
}

unsafe extern "C" fn f2() {
    let s = Arc::new(Ss {
        n: 789,
        m: Mutex::new(()),
    });
    let mut x = X.lock().unwrap();
    *x = Some(s.clone());
    drop(x);

    let mut guard = s.m.lock().unwrap();
    s.n = 789;
    drop(guard);
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let x = X.lock().unwrap();
    *x = None;
    drop(x);

    let handle1 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let x = X.lock().unwrap();
    if let Some(ref s) = *x {
        println!("{}", s.n);
    }
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}