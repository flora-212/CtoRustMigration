use std::sync::{Arc, Mutex};
use std::thread;

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

    fn f1(&self) {
        let x = self.n4;
        let _guard1 = self.m1.lock().unwrap();
        self.n1 += x;
        self.n2 += x;
        let _guard2 = self.m2.lock().unwrap();
        self.n3 += x;
    }
}

#[no_mangle]
pub extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared_data = unsafe { Arc::from_raw(arg as *const SharedData) };
    shared_data.f1();
    Arc::into_raw(shared_data);
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let shared_data = Arc::new(SharedData::new());
    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_data_clone1) as *mut libc::c_void);
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_data_clone2) as *mut libc::c_void);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let shared_data = unsafe { Arc::from_raw(shared_data.as_ptr()) };
    println!("{} {} {} {}", shared_data.n1, shared_data.n2, shared_data.n3, shared_data.n4);

    0
}

pub fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}