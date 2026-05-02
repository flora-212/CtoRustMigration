use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_void;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

#[no_mangle]
pub static SHARED_DATA: Arc<SharedData> = Arc::new(SharedData {
    n1: 0,
    num_mutex: Mutex::new(0),
});

#[no_mangle]
pub extern "C" fn lock(shared_data: &Arc<SharedData>) {
    let _guard = shared_data.num_mutex.lock().unwrap();
}

#[no_mangle]
pub extern "C" fn unlock() {
    // No-op, as the lock is automatically released when the guard goes out of scope
}

#[no_mangle]
pub extern "C" fn f1(shared_data: &Arc<SharedData>) {
    lock(shared_data);
    {
        let mut n1 = shared_data.num_mutex.lock().unwrap();
        *n1 += 1;
    }
    unlock();
}

#[no_mangle]
pub extern "C" fn lock2(shared_data: &Arc<SharedData>, n: i32) -> i32 {
    lock(shared_data);
    {
        let mut n1 = shared_data.num_mutex.lock().unwrap();
        *n1 += n;
    }
    *shared_data.num_mutex.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn unlock2(shared_data: &Arc<SharedData>, n: i32) -> i32 {
    {
        let mut n1 = shared_data.num_mutex.lock().unwrap();
        *n1 += n;
    }
    let n2 = *shared_data.num_mutex.lock().unwrap();
    unlock();
    n2
}

#[no_mangle]
pub extern "C" fn f2(shared_data: &Arc<SharedData>) -> i32 {
    let n2 = lock2(shared_data, 1);
    {
        let mut n1 = shared_data.num_mutex.lock().unwrap();
        *n1 += 1;
    }
    let n2 = unlock2(shared_data, 1);
    n2
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut c_void) -> *mut c_void {
    let shared_data = SHARED_DATA.clone();
    lock(&shared_data);
    f1(&shared_data);
    f2(&shared_data);
    unlock();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let shared_data = SHARED_DATA.clone();
    let id1 = thread::spawn(move || t_fun(std::ptr::null_mut()));
    let id2 = thread::spawn(move || t_fun(std::ptr::null_mut()));

    id1.join().unwrap();
    id2.join().unwrap();

    let n1 = shared_data.num_mutex.lock().unwrap().clone();
    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, n1);
    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}