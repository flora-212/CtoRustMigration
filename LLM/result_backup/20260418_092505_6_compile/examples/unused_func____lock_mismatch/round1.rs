use std::sync::{Arc, Mutex};
use std::thread;

#[no_mangle]
static mut n1: i32 = 0;

#[no_mangle]
static mut num_mutex1: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
static mut num_mutex2: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

#[no_mangle]
unsafe extern "C" fn f1() {
    let mut num_mutex1 = num_mutex1.lock().unwrap();
    *num_mutex1 += 1;
}

#[no_mangle]
unsafe extern "C" fn f2() {
    let mut num_mutex2 = num_mutex2.lock().unwrap();
    *num_mutex2 += 1;
}

#[no_mangle]
unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );

    libc::pthread_join(id1, std::ptr::null_mut());
    libc::pthread_join(id2, std::ptr::null_mut());

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}