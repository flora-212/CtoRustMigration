use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
pub struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<()>,
    m2: Mutex<()>,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            n1: 0,
            n2: 1,
            n3: 2,
            n4: 3,
            m1: Mutex::new(()),
            m2: Mutex::new(()),
        }
    }

    fn f1(&mut self) {
        let x = self.n4;
        let mut m1 = self.m1.lock().unwrap();
        self.n1 += x;
        self.n2 += x;
        drop(m1);
        let mut m2 = self.m2.lock().unwrap();
        self.n3 += x;
        self.n4 += x;
    }
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared_data = arg as *mut SharedData;
    (*shared_data).f1();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let shared_data = Arc::new(Mutex::new(SharedData::new()));

    let mut handles = vec![];

    for _ in 0..2 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            t_fun(&mut *data as *mut SharedData);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = shared_data.lock().unwrap();
    unsafe {
        libc::printf(
            b"%d %d %d %d\n\0".as_ptr(),
            data.n1,
            data.n2,
            data.n3,
            data.n4,
        );
    }

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}