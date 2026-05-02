use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<()>,
    m2: Mutex<()>,
}

static mut S: Option<Arc<SharedData>> = None;
static INIT: Once = Once::new();

fn get_shared_data() -> Arc<SharedData> {
    INIT.call_once(|| {
        unsafe {
            S = Some(Arc::new(SharedData {
                n1: 0,
                n2: 1,
                n3: 2,
                n4: 3,
                m1: Mutex::new(()),
                m2: Mutex::new(()),
            }));
        }
    });
    unsafe { S.as_ref().unwrap().clone() }
}

unsafe extern "C" fn f1() {
    let s = get_shared_data();
    let x = s.n4;
    let _guard1 = s.m1.lock().unwrap();
    s.n1 += x;
    s.n2 += x;
    let _guard2 = s.m2.lock().unwrap();
    s.n3 += x;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = get_shared_data();
    let handle1 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });
    let s = get_shared_data();
    let handle2 = thread::spawn(move || {
        t_fun(ptr::null_mut());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(
        b"%d %d %d %d\n\0".as_ptr() as *const libc::c_char,
        s.n1,
        s.n2,
        s.n3,
        s.n4,
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}