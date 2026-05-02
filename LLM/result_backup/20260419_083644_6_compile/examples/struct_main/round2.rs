use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
}

static SHARED_DATA: Arc<SharedData> = Arc::new(SharedData {
    n1: Mutex::new(0),
});

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let shared_data = &SHARED_DATA;
    let mut lock = shared_data.n1.lock().unwrap();
    *lock += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut handles = vec![];

    for _ in 0..2 {
        let shared_data_clone = SHARED_DATA.clone();
        let handle = thread::spawn(move || {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let shared_data = &SHARED_DATA;
    let mut lock = shared_data.n1.lock().unwrap();
    *lock += 1;

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *lock);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}