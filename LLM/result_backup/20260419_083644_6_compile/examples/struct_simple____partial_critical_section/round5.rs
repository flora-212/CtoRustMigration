use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<i32>,
    m2: Mutex<i32>,
}

static mut S: Option<Arc<Mutex<SharedData>>> = None;

fn init_shared_data() -> Arc<Mutex<SharedData>> {
    unsafe {
        if S.is_none() {
            S = Some(Arc::new(Mutex::new(SharedData {
                n1: 0,
                n2: 1,
                n3: 2,
                n4: 3,
                m1: Mutex::new(0),
                m2: Mutex::new(0),
            })));
        }
        S.as_ref().unwrap().clone()
    }
}

fn f1(shared_data: Arc<Mutex<SharedData>>) {
    let x = {
        let data = shared_data.lock().unwrap();
        data.n4
    };

    {
        let mut data = shared_data.lock().unwrap();
        data.n1 += x;
        data.n2 += x;
    }

    {
        let mut data = shared_data.lock().unwrap();
        data.n3 += x;
        data.n4 += x;
    }
}

fn t_fun(shared_data: Arc<Mutex<SharedData>>) {
    f1(shared_data);
}

fn main_0() -> libc::c_int {
    let shared_data = init_shared_data();

    let handle1 = thread::spawn({
        let shared_data = shared_data.clone();
        move || t_fun(shared_data)
    });

    let handle2 = thread::spawn({
        let shared_data = shared_data.clone();
        move || t_fun(shared_data)
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = shared_data.lock().unwrap();
    unsafe {
        libc::printf(
            b"%d %d %d %d\n\0".as_ptr() as *const libc::c_char,
            data.n1,
            data.n2,
            data.n3,
            data.n4,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}