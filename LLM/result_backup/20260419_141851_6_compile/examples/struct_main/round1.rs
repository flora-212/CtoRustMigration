use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<()>,
}

static mut SHARED_DATA: Option<Arc<SharedData>> = None;

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let data = SHARED_DATA.as_ref().unwrap();
    let _guard = data.m1.lock().unwrap();
    data.n1 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let data = Arc::new(SharedData {
        n1: 0,
        m1: Mutex::new(()),
    });
    SHARED_DATA = Some(data.clone());

    let mut handles = vec![];

    for _ in 0..2 {
        let data_clone = data.clone();
        let handle = thread::spawn(move || {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = SHARED_DATA.as_ref().unwrap();
    let _guard = data.m1.lock().unwrap();
    data.n1 += 1;

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, data.n1);
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}