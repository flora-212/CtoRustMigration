use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[derive(Debug)]
struct Shared {
    n1: i32,
    m1: Mutex<i32>,
    cond: Condvar,
}

static S: Arc<Shared> = Arc::new(Shared {
    n1: 0,
    m1: Mutex::new(0),
    cond: Condvar::new(),
});

unsafe extern "C" fn f1() {
    let s = S.clone();
    let mut n1 = s.m1.lock().unwrap();
    *n1 += 1;
    if *n1 == 1 {
        n1 = s.cond.wait(n1).unwrap();
    } else {
        s.cond.notify_one();
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    let t1 = thread::spawn(|| t_fun(std::ptr::null_mut()));
    let t2 = thread::spawn(|| t_fun(std::ptr::null_mut()));

    t1.join().unwrap();
    t2.join().unwrap();

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
