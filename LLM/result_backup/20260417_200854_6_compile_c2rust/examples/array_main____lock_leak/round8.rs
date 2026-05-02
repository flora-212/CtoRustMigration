use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
use std::ptr;

const N: usize = 5;

#[no_mangle]
static mut n1: [i32; N] = [0; N];

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
        let mut num = num_mutex[i].lock().unwrap();
        *num += 1;
    }
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    for i in 0..N {
        n1[i] += 1;
    }

    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            unsafe { t_fun(ptr::null_mut()) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..N {
        n1[i] += 1;
    }

    let output = format!(
        "{} {} {} {} {}\n",
        n1[0], n1[1], n1[2], n1[3], n1[4]
    );
    let c_string = CString::new(output).unwrap();
    unsafe {
        libc::printf(c_string.as_ptr());
    }

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}