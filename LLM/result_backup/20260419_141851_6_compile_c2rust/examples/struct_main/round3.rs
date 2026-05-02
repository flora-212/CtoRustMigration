use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<()>,
}

static SHARED_DATA: Arc<SharedData> = Arc::new(SharedData {
    n1: 0,
    m1: Mutex::new(()),
});

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let data = SHARED_DATA.clone();
    let mut guard = data.m1.lock().unwrap();
    data.n1 += 1;
    drop(guard);
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let data = SHARED_DATA.clone();

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

    let mut guard = SHARED_DATA.m1.lock().unwrap();
    SHARED_DATA.n1 += 1;
    drop(guard);

    let c_string = CString::new(format!("{}\n", SHARED_DATA.n1)).unwrap();
    libc::printf(c_string.as_ptr());
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}