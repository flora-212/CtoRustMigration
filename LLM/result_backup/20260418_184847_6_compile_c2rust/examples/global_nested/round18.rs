use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

static INIT: Once = Once::new();
static mut SHARED_DATA: *const Arc<SharedData> = ptr::null();

struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
}

fn get_shared_data() -> Arc<SharedData> {
    unsafe {
        INIT.call_once(|| {
            SHARED_DATA = Box::leak(Box::new(Arc::new(SharedData {
                n1: Mutex::new(0),
                n2: Mutex::new(0),
            })));
        });
        SHARED_DATA.as_ref().unwrap().clone()
    }
}

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let shared_data = get_shared_data();
    {
        let mut n1 = shared_data.n1.lock().unwrap();
        *n1 += 1;
    }
    {
        let mut n2 = shared_data.n2.lock().unwrap();
        *n2 += 1;
    }
    {
        let mut n1 = shared_data.n1.lock().unwrap();
        *n1 += 1;
    }
    {
        let mut n2 = shared_data.n2.lock().unwrap();
        let n1 = shared_data.n1.lock().unwrap();
        *n1 += *n2;
    }
    {
        let mut n1 = shared_data.n1.lock().unwrap();
        *n1 += 1;
    }
    {
        let mut n2 = shared_data.n2.lock().unwrap();
        *n2 += 1;
    }
    {
        let mut n1 = shared_data.n1.lock().unwrap();
        let n2 = shared_data.n2.lock().unwrap();
        *n2 += *n1;
    }
    {
        let mut n2 = shared_data.n2.lock().unwrap();
        *n2 += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let shared_data = get_shared_data();
    let handle1 = thread::spawn(move || {
        f1();
    });
    let handle2 = thread::spawn(move || {
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = shared_data.n1.lock().unwrap();
    let n2 = shared_data.n2.lock().unwrap();
    libc::printf(b"%d %d\n\0".as_ptr() as *const libc::c_char, *n1, *n2);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}