use std::sync::{Arc, Mutex};
use std::thread;

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
        let mut value = self.mutexes[index].lock().unwrap();
        *value += 1;
        self.values[index] = *value;
    }
}

#[no_mangle]
pub extern "C" fn f1(mutex_array: &MutexArray) {
    mutex_array.increment(0);
    mutex_array.increment(1);
    mutex_array.increment(0);
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let mutex_array = unsafe { &*(arg as *const MutexArray) };
    f1(mutex_array);
    std::ptr::null_mut()
}

unsafe fn main_0() -> std::ffi::c_int {
    let mutex_array = Arc::new(MutexArray::new());
    let mutex_array_clone = Arc::clone(&mutex_array);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(mutex_array_clone) as *mut std::ffi::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(Arc::clone(&mutex_array)) as *mut std::ffi::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let result = mutex_array.lock().unwrap();
    println!("{} {} {}", result.values[0], result.values[1], result.values[2]);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}