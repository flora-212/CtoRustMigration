use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<i32>,
}

static LOCK: Mutex<()> = Mutex::new(());

static X: Arc<Mutex<Option<Arc<Ss>>>> = Arc::new(Mutex::new(None));

unsafe extern "C" fn f1() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(123),
    });
    let mut s_clone = s.clone();
    *X.lock().unwrap() = Some(s);

    let mut n = s_clone.m.lock().unwrap();
    *n = 456;
}

unsafe extern "C" fn f2() {
    let s = Arc::new(Ss {
        n: 123,
        m: Mutex::new(123),
    });
    let mut s_clone = s.clone();
    *X.lock().unwrap() = Some(s);

    let _guard = LOCK.lock().unwrap();
    let mut n = s_clone.m.lock().unwrap();
    *n = 789;
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

    if let Some(ref x) = *X.lock().unwrap() {
        println!("{}", x.m.lock().unwrap());
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}