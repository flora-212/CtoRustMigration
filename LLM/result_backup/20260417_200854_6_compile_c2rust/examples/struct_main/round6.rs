use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;
use std::sync::Once;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    m1: Mutex<i32>,
}

static INIT: Once = Once::new();
static mut SHARED_DATA: Option<Arc<SharedData>> = None;

fn get_shared_data() -> Arc<SharedData> {
    unsafe {
        INIT.call_once(|| {
            SHARED_DATA = Some(Arc::new(SharedData {
                n1: 0,
                m1: Mutex::new(0),
            }));
        });
        SHARED_DATA.as_ref().unwrap().clone()
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let data = get_shared_data();
    let mut guard = data.m1.lock().unwrap();
    *guard += 1;
    drop(guard);
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let data = get_shared_data();
    let mut guard = data.m1.lock().unwrap();
    *guard += 1;
    drop(guard);

    let data_clone = data.clone();
    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let data_clone = data.clone();
    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = get_shared_data();
    let mut guard = data.m1.lock().unwrap();
    *guard += 1;
    drop(guard);

    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, data.n1);
    }

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}