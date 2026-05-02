use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
}

static mut SHARED_DATA: Option<Arc<SharedData>> = None;
static INIT: Once = Once::new();

fn get_shared_data() -> Arc<SharedData> {
    INIT.call_once(|| {
        unsafe {
            SHARED_DATA = Some(Arc::new(SharedData {
                n1: Mutex::new(0),
            }));
        }
    });
    unsafe { SHARED_DATA.as_ref().unwrap().clone() }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let shared_data = get_shared_data();
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
        let shared_data_clone = get_shared_data();
        let handle = thread::spawn(move || {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let shared_data = get_shared_data();
    let mut lock = shared_data.n1.lock().unwrap();
    *lock += 1;

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, *lock);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}