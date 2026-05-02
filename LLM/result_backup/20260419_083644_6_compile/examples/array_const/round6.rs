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
            mutexes: [Mutex::new(0); 3],
            values: [0; 3],
        }
    }

    fn increment(&self, index: usize) {
        let mut lock = self.mutexes[index].lock().unwrap();
        *lock += 1;
    }

    fn print(&self) {
        println!("{} {} {}", self.values[0], self.values[1], self.values[2]);
    }
}

#[no_mangle]
pub extern "C" fn f1(mutex_array: &MutexArray) {
    for i in 0..3 {
        mutex_array.increment(i);
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let mutex_array = unsafe { &*(arg as *const MutexArray) };
    f1(mutex_array);
    std::ptr::null_mut()
}

unsafe fn main_0() -> i32 {
    let mutex_array = Arc::new(MutexArray::new());

    let mut handles = vec![];

    for _ in 0..2 {
        let mutex_array_clone = Arc::clone(&mutex_array);
        let handle = thread::spawn(move || {
            t_fun(&mutex_array_clone as *const MutexArray as *mut std::ffi::c_void);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    mutex_array.lock().unwrap().print();

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}