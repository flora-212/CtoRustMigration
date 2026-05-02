use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct NumMutex {
    value: i32,
    mutex: Mutex<()>,
}

static NUM_MUTEX: NumMutex = NumMutex {
    value: 0,
    mutex: Mutex::new(()),
};

#[no_mangle]
pub extern "C" fn inc() -> i32 {
    let mut num_mutex = NUM_MUTEX.mutex.lock().unwrap();
    NUM_MUTEX.value += 1;
    if NUM_MUTEX.value != 0 {
        NUM_MUTEX.value
    } else {
        NUM_MUTEX.value + 1
    }
}

#[no_mangle]
pub extern "C" fn f1() {
    inc();
}

#[no_mangle]
pub extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let handle1 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });
    let handle2 = thread::spawn(|| {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, NUM_MUTEX.value);
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}