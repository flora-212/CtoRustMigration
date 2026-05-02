use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;
use std::ffi::CString;

struct SharedData {
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

static INIT: Once = Once::new();
static mut S: Option<Arc<Mutex<SharedData>>> = None;

#[no_mangle]
pub unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = S.as_ref().unwrap().clone();
    let mut s = s.lock().unwrap();
    s.f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(|| {
        S = Some(Arc::new(Mutex::new(SharedData::new())));
    });

    let s = S.as_ref().unwrap().clone();
    let handle1 = thread::spawn(move || {
        let mut s = s.lock().unwrap();
        s.f1();
    });

    let s = S.as_ref().unwrap().clone();
    let handle2 = thread::spawn(move || {
        let mut s = s.lock().unwrap();
        s.f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.as_ref().unwrap().lock().unwrap();
    libc::printf(
        CString::new("%d %d %d %d\n").unwrap().as_ptr(),
        s.n1,
        s.n2,
        s.n3,
        s.n4,
    );

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}