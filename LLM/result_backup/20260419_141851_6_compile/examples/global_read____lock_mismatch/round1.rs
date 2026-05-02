use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[no_mangle]
static mut n1: i32 = 0;
#[no_mangle]
static mut n2: i32 = 0;

#[no_mangle]
static mut num_mutex1: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
#[no_mangle]
static mut num_mutex2: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
unsafe extern "C" fn f1() {
    let mut num_mutex1 = num_mutex1.lock().unwrap();
    n1 = n1 + n2;
}

#[no_mangle]
unsafe extern "C" fn f2() {
    let mut num_mutex2 = num_mutex2.lock().unwrap();
    n1 = n1 + n2;
}

#[no_mangle]
unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    if arg as libc::c_long == 0 {
        f1();
    } else {
        f2();
    }
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let num_mutex1 = Arc::clone(&num_mutex1);
    let num_mutex2 = Arc::clone(&num_mutex2);

    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        t_fun(1 as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(b"%d %d\n\0".as_ptr(), n1, n2);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}