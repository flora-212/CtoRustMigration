use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::c_void;
use std::sync::Once;

static mut N1: i32 = 0;
static mut N2: i32 = 0;
static mut N3: i32 = 1;

static NUM_MUTEX: Once = Once::new();
static mut NUM_MUTEX_INSTANCE: *const Arc<Mutex<i32>> = ptr::null();

fn get_num_mutex() -> &'static Arc<Mutex<i32>> {
    unsafe {
        NUM_MUTEX.call_once(|| {
            NUM_MUTEX_INSTANCE = Box::leak(Box::new(Arc::new(Mutex::new(0))));
        });
        &*NUM_MUTEX_INSTANCE
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let x = N3;
    let mut num_mutex = get_num_mutex().lock().unwrap();
    *num_mutex += x;
    N1 += x;
    N2 += x;
    N3 += x;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut c_void) -> *mut c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let num_mutex_clone = Arc::clone(get_num_mutex());

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        N1,
        N2,
        N3,
    );

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}