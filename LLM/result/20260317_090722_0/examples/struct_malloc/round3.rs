use ::libc;
use std::ptr;
use std::sync::{Arc, Mutex, Condvar};

#[derive(Clone)]
#[repr(C)]
pub struct ss {
    pub n: Arc<Mutex<libc::c_int>>,
    pub m: Arc<Mutex<libc::c_int>>,
    pub c: Arc<Condvar>,
}

extern "C" {
    fn pthread_create(
        __newthread: *mut libc::c_ulong,
        __attr: *const libc::c_void,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: libc::c_ulong, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
}

unsafe extern "C" fn f1(s: *mut ss) {
    let s = &mut *s;
    let mut n = s.n.lock().unwrap();
    *n += 1;
    if *n == 1 {
        while *n == 1 {
            n = s.c.wait(n).unwrap();
        }
    } else {
        s.c.notify_one();
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(arg as *mut ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s = Arc::new(ss {
        n: Arc::new(Mutex::new(0)),
        m: Arc::new(Mutex::new(0)),
        c: Arc::new(Condvar::new()),
    });

    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);

    pthread_create(
        &mut id1,
        ptr::null(),
        Some(t_fun),
        Arc::into_raw(s1) as *mut libc::c_void,
    );
    pthread_create(
        &mut id2,
        ptr::null(),
        Some(t_fun),
        Arc::into_raw(s2) as *mut libc::c_void,
    );

    pthread_join(id1, ptr::null_mut());
    pthread_join(id2, ptr::null_mut());

    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
