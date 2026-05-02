use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::os::raw::c_int;
use std::sync::Once;

const N: usize = 5;

#[no_mangle]
static mut n1: [c_int; N] = [0; N];
#[no_mangle]
static mut n2: [c_int; N] = [1, 2, 3, 4, 5];
#[no_mangle]
static mut n3: [c_int; N] = [1; N];

static NUM_MUTEX: Once = Once::new();
static mut NUM_MUTEX_INIT: [Arc<Mutex<()>>; N] = [Arc::new(Mutex::new(())); N];

fn get_num_mutex() -> &'static [Arc<Mutex<()>>; N] {
    NUM_MUTEX.call_once(|| {
        unsafe {
            NUM_MUTEX_INIT = [
                Arc::new(Mutex::new(())),
                Arc::new(Mutex::new(())),
                Arc::new(Mutex::new(())),
                Arc::new(Mutex::new(())),
                Arc::new(Mutex::new(())),
            ];
        }
    });
    unsafe { &NUM_MUTEX_INIT }
}

unsafe extern "C" fn f1() {
    let num_mutex = get_num_mutex();
    for i in 0..N {
        let x = n3[i];
        let mutex = &num_mutex[i];
        let _guard = mutex.lock().unwrap();
        n1[i] += x;
        n2[i] += x;
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> c_int {
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

    let print = |arr: &[c_int; N]| {
        let c_str = CString::new(format!(
            "{} {} {} {} {}\n",
            arr[0], arr[1], arr[2], arr[3], arr[4]
        ))
        .unwrap();
        libc::printf(c_str.as_ptr());
    };

    print(&n1);
    print(&n2);
    print(&n3);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}