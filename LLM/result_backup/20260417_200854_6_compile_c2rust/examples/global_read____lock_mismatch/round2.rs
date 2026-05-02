use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
static mut N1: i32 = 0;
#[no_mangle]
static mut N2: i32 = 0;

#[no_mangle]
static NUM_MUTEX1: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
#[no_mangle]
static NUM_MUTEX2: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut num_mutex1 = NUM_MUTEX1.lock().unwrap();
    *num_mutex1 = *num_mutex1 + N2;
}

#[no_mangle]
pub unsafe extern "C" fn f2() {
    let mut num_mutex2 = NUM_MUTEX2.lock().unwrap();
    *num_mutex2 = *num_mutex2 + N2;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    if arg as libc::c_long == 0 {
        f1();
    } else {
        f2();
    }
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let id1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });
    let id2 = thread::spawn(|| {
        t_fun(1 as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d\n\0".as_ptr() as *const c_char,
            N1,
            N2,
        );
    }
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}