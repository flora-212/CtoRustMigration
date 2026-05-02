use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::sync::Once;

static INIT: Once = Once::new();
static mut SHARED_DATA: Option<Arc<Mutex<SharedData>>> = None;

#[derive(Debug)]
struct SharedData {
    n1: i32,
}

fn get_shared_data() -> Arc<Mutex<SharedData>> {
    unsafe {
        INIT.call_once(|| {
            SHARED_DATA = Some(Arc::new(Mutex::new(SharedData {
                n1: 0,
            })));
        });
        SHARED_DATA.as_ref().unwrap().clone()
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let data = get_shared_data();
    let mut guard = data.lock().unwrap();
    guard.n1 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let data = get_shared_data();
    let mut guard = data.lock().unwrap();
    guard.n1 += 1;

    let data_clone = data.clone();
    let handle1 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    let data_clone = data.clone();
    let handle2 = thread::spawn(move || {
        unsafe { t_fun(std::ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = get_shared_data();
    let mut guard = data.lock().unwrap();
    guard.n1 += 1;

    let c_string = CString::new(format!("{}\n", guard.n1)).unwrap();
    libc::printf(c_string.as_ptr());

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
