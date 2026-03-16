use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static NUM_MUTEX: Mutex<()> = Mutex::new(());

fn inc() {
    unsafe {
        N1 += 1;
    }
}

fn f1() {
    let _guard = NUM_MUTEX.lock().unwrap();
    inc();
}

extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    let handle1 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let handle2 = thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
