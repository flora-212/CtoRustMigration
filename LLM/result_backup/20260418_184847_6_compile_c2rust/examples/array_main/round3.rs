use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::Once;

const N: c_int = 5;

#[no_mangle]
static mut n1: [c_int; 5] = [0; 5];

#[no_mangle]
static NUM_MUTEX: OnceLock<[Arc<Mutex<c_int>>; 5]> = OnceLock::new();

static INIT: Once = Once::new();

unsafe extern "C" fn f1() {
    let num_mutex = NUM_MUTEX.get().unwrap();
    for i in 0..N {
        let mut num = num_mutex[i as usize].lock().unwrap();
        *num += 1;
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
    INIT.call_once(|| {
        for i in 0..N {
            n1[i as usize] += 1;
        }
    });

    NUM_MUTEX.get_or_init(|| {
        [
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
        ]
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            f1();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..N {
        n1[i as usize] += 1;
    }

    let output = format!(
        "{} {} {} {} {}\n",
        n1[0], n1[1], n1[2], n1[3], n1[4]
    );
    let c_string = CString::new(output).unwrap();
    libc::printf(c_string.as_ptr());

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}