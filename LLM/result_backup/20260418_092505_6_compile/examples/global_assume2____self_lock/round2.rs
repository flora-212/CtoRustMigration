use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

static INIT: Once = Once::new();
static mut N1: i32 = 0;
static mut NUM_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

fn init_globals() {
    unsafe {
        NUM_MUTEX = Arc::new(Mutex::new(0));
    }
}

#[no_mangle]
pub unsafe extern "C" fn inc() -> i32 {
    INIT.call_once(init_globals);
    N1 += 1;
    if N1 != 0 {
        N1
    } else {
        N1 + 1
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let num_mutex_clone = NUM_MUTEX.clone();
    let num_mutex_clone2 = NUM_MUTEX.clone();
    let _guard1 = num_mutex_clone.lock().unwrap();
    let _guard2 = num_mutex_clone2.lock().unwrap();
    inc();
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    INIT.call_once(init_globals);
    let num_mutex_clone = NUM_MUTEX.clone();
    let num_mutex_clone2 = NUM_MUTEX.clone();

    let handle1 = thread::spawn(move || {
        let _guard = num_mutex_clone.lock().unwrap();
        t_fun(ptr::null_mut());
    });

    let handle2 = thread::spawn(move || {
        let _guard = num_mutex_clone2.lock().unwrap();
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", N1);
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}