use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::ptr;

const N: usize = 5;

#[no_mangle]
static mut n1: [i32; N] = [0; N];

#[no_mangle]
static mut n2: [i32; N] = [1, 2, 3, 4, 5];

#[no_mangle]
static mut n3: [i32; N] = [1; N];

#[no_mangle]
static mut num_mutex: [Arc<Mutex<i32>>; N] = [
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
    Arc::new(Mutex::new(0)),
];

unsafe extern "C" fn f1() {
    for i in 0..N {
        let x = n3[i];
        let mut num_mutex_i = num_mutex[i].lock().unwrap();
        n1[i] += x;
        n2[i] += x;
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
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

    let print = |arr: &[i32]| {
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