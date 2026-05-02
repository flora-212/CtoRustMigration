use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss0 {
    pub n1: i32,
    pub n2: i32,
}

#[derive(Clone)]
pub struct ss {
    pub s: ss0,
    pub m: Arc<Mutex<()>>,
}

#[no_mangle]
pub static mut s: ss = ss {
    s: ss0 { n1: 0, n2: 1 },
    m: Arc::new(Mutex::new(())),
};

#[no_mangle]
pub unsafe extern "C" fn f1() {
    let _guard = s.m.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let _guard = s.m.lock().unwrap();
    f1();
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    let s_clone = Arc::clone(&s);
    let handle1 = thread::spawn(move || {
        t_fun(libc::NULL);
    });

    let s_clone = Arc::clone(&s);
    let handle2 = thread::spawn(move || {
        t_fun(libc::NULL);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(
        b"%d %d\n\0".as_ptr() as *const libc::c_char,
        s.s.n1,
        s.s.n2,
    );

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}