use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static NUM_MUTEX: Mutex<()> = Mutex::new(());

fn inc() -> i32 {
    unsafe {
        N1 += 1;
        if N1 != 0 { N1 } else { N1 + 1 }
    }
}

fn f1() {
    let _guard = NUM_MUTEX.lock().unwrap();
    inc();
}

fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

fn main_0() -> i32 {
    let mut id1 = None;
    let mut id2 = None;

    id1 = Some(thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    }));

    id2 = Some(thread::spawn(|| {
        unsafe { t_fun(std::ptr::null_mut()) };
    }));

    id1.take().unwrap().join().unwrap();
    id2.take().unwrap().join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
