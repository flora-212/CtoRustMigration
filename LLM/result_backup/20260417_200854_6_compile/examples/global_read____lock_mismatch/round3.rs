use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Once;

static mut N1: i32 = 0;
static mut N2: i32 = 0;

static NUM_MUTEX1: Once = Once::new();
static NUM_MUTEX2: Once = Once::new();

static mut NUM_MUTEX1_PTR: *const Arc<Mutex<i32>> = ptr::null();
static mut NUM_MUTEX2_PTR: *const Arc<Mutex<i32>> = ptr::null();

fn get_num_mutex1() -> &'static Arc<Mutex<i32>> {
    unsafe {
        NUM_MUTEX1.call_once(|| {
            NUM_MUTEX1_PTR = Box::leak(Box::new(Arc::new(Mutex::new(0))));
        });
        &*NUM_MUTEX1_PTR
    }
}

fn get_num_mutex2() -> &'static Arc<Mutex<i32>> {
    unsafe {
        NUM_MUTEX2.call_once(|| {
            NUM_MUTEX2_PTR = Box::leak(Box::new(Arc::new(Mutex::new(0))));
        });
        &*NUM_MUTEX2_PTR
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let mut num_mutex1 = get_num_mutex1().lock().unwrap();
    *num_mutex1 = *num_mutex1 + N2;
}

#[no_mangle]
pub unsafe extern "C" fn f2() {
    let mut num_mutex2 = get_num_mutex2().lock().unwrap();
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