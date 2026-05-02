use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss0 {
    pub n1: i32,
    pub n2: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss {
    pub s: ss0,
    pub m: Arc<Mutex<()>>,
}

#[no_mangle]
pub static mut s: ss = ss {
    s: ss0 {
        n1: 0,
        n2: 1,
    },
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
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );

    libc::pthread_join(id1, ptr::null_mut());
    libc::pthread_join(id2, ptr::null_mut());

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