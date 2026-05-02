use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

static mut N1: i32 = 0;

static NUM_MUTEX1: Once = Once::new();
static NUM_MUTEX2: Once = Once::new();

fn get_num_mutex1() -> Arc<Mutex<i32>> {
    NUM_MUTEX1.call_once(|| {
        unsafe {
            NUM_MUTEX1 = Once::new();
        }
    });
    unsafe { NUM_MUTEX1.get().unwrap().clone() }
}

fn get_num_mutex2() -> Arc<Mutex<i32>> {
    NUM_MUTEX2.call_once(|| {
        unsafe {
            NUM_MUTEX2 = Once::new();
        }
    });
    unsafe { NUM_MUTEX2.get().unwrap().clone() }
}

#[no_mangle]
unsafe extern "C" fn f1() {
    let mut num_mutex1 = get_num_mutex1().lock().unwrap();
    *num_mutex1 += 1;
}

#[no_mangle]
unsafe extern "C" fn f2() {
    let mut num_mutex2 = get_num_mutex2().lock().unwrap();
    *num_mutex2 += 1;
}

#[no_mangle]
unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let num_mutex1 = get_num_mutex1();
    let num_mutex2 = get_num_mutex2();

    let handle1 = thread::spawn(move || {
        f1();
    });

    let handle2 = thread::spawn(move || {
        f2();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}