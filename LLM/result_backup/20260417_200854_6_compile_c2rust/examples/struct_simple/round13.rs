use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;
use libc;
use std::sync::OnceLock;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<i32>,
    m2: Mutex<i32>,
}

static S: OnceLock<Arc<Mutex<SharedData>>> = OnceLock::new();

fn init_shared_data() -> Arc<Mutex<SharedData>> {
    S.get_or_init(|| Arc::new(Mutex::new(SharedData {
        n1: 0,
        n2: 1,
        n3: 2,
        n4: 3,
        m1: Mutex::new(0),
        m2: Mutex::new(0),
    }))).clone()
}

fn f1(shared_data: Arc<Mutex<SharedData>>) {
    let x = {
        let data = shared_data.lock().unwrap();
        data.n4
    };
    {
        let mut data = shared_data.lock().unwrap();
        *data.m1.lock().unwrap() += x;
    }
    {
        let mut data = shared_data.lock().unwrap();
        *data.m2.lock().unwrap() += x;
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared_data = init_shared_data();
    f1(shared_data);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let shared_data = init_shared_data();

    let handle1 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
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