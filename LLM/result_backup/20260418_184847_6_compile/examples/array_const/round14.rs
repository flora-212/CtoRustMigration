use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

#[derive(Debug)]
struct MutexArray {
    mutexes: [Mutex<i32>; 3],
    values: [i32; 3],
}

impl MutexArray {
    fn new() -> Self {
        MutexArray {
            mutexes: [Mutex::new(0), Mutex::new(0), Mutex::new(0)],
            values: [0; 3],
        }
    }

    fn increment(&self, index: usize) {
        let mut lock = self.mutexes[index].lock().unwrap();
        *lock += 1;
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let array = unsafe { Arc::from_raw(arg as *const MutexArray) };
    array.increment(0);
    array.increment(1);
    array.increment(2);
    Arc::into_raw(array);
    std::ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let array = Arc::new(MutexArray::new());

    let array_clone1 = array.clone();
    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(array_clone1) as *mut libc::c_void);
    });

    let array_clone2 = array.clone();
    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(array_clone2) as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let result = array.lock().unwrap();
    unsafe {
        libc::printf(
            CString::new("%d %d %d\n").unwrap().as_ptr(),
            result.values[0],
            result.values[1],
            result.values[2],
        );
    }

    0
}

pub fn main() {
    std::process::exit(main_0() as i32);
}