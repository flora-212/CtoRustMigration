use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
}

fn f1(shared: Arc<SharedData>) {
    let mut n1 = shared.n1.lock().unwrap();
    *n1 += 1;
    let mut n2 = shared.n2.lock().unwrap();
    *n2 += 1;
    *n1 += 1;
    *n1 += *n2;
    *n1 += 1;
    *n2 += 1;
    *n2 += *n1;
    *n2 += 1;
}

fn f2(shared: Arc<SharedData>) {
    let mut n2 = shared.n2.lock().unwrap();
    *n2 += 1;
    let mut n1 = shared.n1.lock().unwrap();
    *n1 += 1;
    *n2 += 1;
    *n2 += *n1;
    *n2 += 1;
    *n1 += 1;
    *n1 += *n2;
    *n1 += 1;
}

fn t_fun(arg: *mut libc::c_void, shared: Arc<SharedData>) {
    if arg as libc::c_long == 0 {
        f1(shared);
    } else {
        f2(shared);
    }
}

fn main_0() -> libc::c_int {
    let shared = Arc::new(SharedData {
        n1: Mutex::new(0),
        n2: Mutex::new(0),
    });

    let shared_clone1 = Arc::clone(&shared);
    let shared_clone2 = Arc::clone(&shared);

    let handle1 = thread::spawn(move || t_fun(std::ptr::null_mut(), shared_clone1));
    let handle2 = thread::spawn(move || t_fun(1 as *mut libc::c_void, shared_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared.n1.lock().unwrap();
    let n2 = shared.n2.lock().unwrap();
    unsafe {
        libc::printf(b"%d %d\n\0".as_ptr() as *const libc::c_char, *n1, *n2);
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}