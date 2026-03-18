use std::sync::{Arc, Mutex};
use std::thread;
use libc;

#[derive(Debug)]
pub struct SharedData {
    n1: i32,
    m1: Mutex<()>,
}

static mut S: Option<Arc<SharedData>> = None;

unsafe extern "C" fn f1() {
    let s = S.as_ref().unwrap();
    let _guard = s.m1.lock().unwrap();
    s.n1 += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    S = Some(Arc::new(SharedData {
        n1: 0,
        m1: Mutex::new(()),
    }));

    let s = S.as_ref().unwrap();
    s.n1 += 1;

    let attr = 0 as *const libc::c_void as *const libc::pthread_attr_t;
    let start_routine = Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void);
    let arg = 0 as *mut libc::c_void;

    libc::pthread_create(&mut id1 as *mut libc::c_ulong, attr, start_routine, arg);
    libc::pthread_create(&mut id2 as *mut libc::c_ulong, attr, start_routine, arg);

    libc::pthread_join(id1, 0 as *mut *mut libc::c_void);
    libc::pthread_join(id2, 0 as *mut *mut libc::c_void);

    let s = S.as_ref().unwrap();
    s.n1 += 1;

    0 as libc::c_int
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
