use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

static INIT: Once = Once::new();
static mut S: Arc<Mutex<SharedData>> = Arc::new(Mutex::new(SharedData::new()));

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

#[no_mangle]
pub unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s = S.clone();
    let mut s = s.lock().unwrap();
    s.f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(|| {
        S = Arc::new(Mutex::new(SharedData::new()));
    });

    let s = S.clone();
    let handle1 = thread::spawn(move || {
        let mut s = s.lock().unwrap();
        s.f1();
    });

    let s = S.clone();
    let handle2 = thread::spawn(move || {
        let mut s = s.lock().unwrap();
        s.f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = S.lock().unwrap();
    libc::printf(
        b"%d %d %d %d\n\0".as_ptr(),
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