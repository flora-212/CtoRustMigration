use std::sync::{Mutex, Arc};
use std::thread;

static mut N1: [i32; 5] = [0; 5];
static NUM_MUTEX: [Mutex<()>; 5] = [
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
];

fn f1() {
    for i in 0..5 {
        let _guard = NUM_MUTEX[i].lock().unwrap();
        unsafe {
            N1[i] += 1;
        }
    }
}

extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    for i in 0..5 {
        N1[i] += 1;
    }

    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    for i in 0..5 {
        N1[i] += 1;
    }

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
