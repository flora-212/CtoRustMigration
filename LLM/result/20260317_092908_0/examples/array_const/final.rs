use ::libc;
use std::sync::{Mutex, Once};
use std::thread;

static mut N1: [i32; 3] = [0; 3];
static NUM_MUTEX: [Mutex<()>; 3] = [
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
];

static INIT: Once = Once::new();

unsafe extern "C" fn f1() {
    let _guard1 = NUM_MUTEX[0].lock().unwrap();
    N1[0] += 1;
    let _guard2 = NUM_MUTEX[1].lock().unwrap();
    N1[1] += 1;
    let _guard3 = NUM_MUTEX[2].lock().unwrap();
    N1[2] += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(|| {
        // Initialize any necessary resources here
    });

    let handle1 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    let handle2 = thread::spawn(|| {
        t_fun(std::ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
