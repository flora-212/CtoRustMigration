use ::libc;
use std::ptr;
use std::sync::Mutex;

#[derive(Clone)]
#[repr(C)]
pub struct ss {
    pub n: libc::c_int,
    pub m: Mutex<()>,
}

pub extern "C" fn inc(s: *mut ss) {
    unsafe {
        (*s).n += 1;
    }
}

pub extern "C" fn f1(s: *mut ss) {
    unsafe {
        let mutex = &(*s).m;
        let _guard = mutex.lock().unwrap();
        inc(s);
    }
}

pub extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        f1(arg as *mut ss);
    }
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Box::new(ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s_ptr: *mut ss = Box::into_raw(s);

    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        ptr::null(),
        Some(t_fun),
        s_ptr as *mut libc::c_void,
    );
    libc::pthread_create(
        &mut id2,
        ptr::null(),
        Some(t_fun),
        s_ptr as *mut libc::c_void,
    );

    libc::pthread_join(id1, ptr::null_mut());
    libc::pthread_join(id2, ptr::null_mut());

    Box::from_raw(s_ptr); // Ensure the box is dropped
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
