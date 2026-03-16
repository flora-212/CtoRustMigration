use std::sync::{Mutex, Once};
use std::thread;

static N1: [Mutex<i32>; 5] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];
static INIT: Once = Once::new();

unsafe extern "C" fn f1() {
    for i in 0..5 {
        let mut num = N1[i].lock().unwrap();
        *num += 1;
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    INIT.call_once(|| {
        for i in 0..5 {
            let mut num = N1[i].lock().unwrap();
            *num += 1;
        }
    });

    let handle1 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    for i in 0..5 {
        let mut num = N1[i].lock().unwrap();
        *num += 1;
    }

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}
