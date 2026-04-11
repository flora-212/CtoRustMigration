use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::sync::OnceLock;

const N: usize = 5;

#[no_mangle]
static mut n1: [i32; N] = [0; N];

#[no_mangle]
static NUM_MUTEX: OnceLock<[Arc<Mutex<i32>>; N]> = OnceLock::new();

#[no_mangle]
unsafe extern "C" fn f1() {
    let num_mutex = NUM_MUTEX.get().unwrap();
    for i in 0..N {
        let mut num = num_mutex[i].lock().unwrap();
        *num += 1;
    }
}

#[no_mangle]
unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    NUM_MUTEX.get_or_init(|| {
        [
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
        ]
    });

    for i in 0..N {
        n1[i] += 1;
    }

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
        n1[i] += 1;
    }

    libc::printf(
        b"%d %d %d %d %d\n\0".as_ptr() as *const libc::c_char,
        n1[0],
        n1[1],
        n1[2],
        n1[3],
        n1[4],
    );

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
