use ::libc;
use std::ptr;
use std::sync::{Arc, Mutex};

extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}

type pthread_t = libc::c_ulong;
type pthread_attr_t = [libc::c_char; 56];

#[derive(Clone)]
#[repr(C)]
pub struct ss {
    pub n: libc::c_int,
    pub m: Arc<Mutex<()>>,
}

lazy_static::lazy_static! {
    static ref S1: Arc<ss> = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });
    static ref S2: Arc<ss> = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });
}

unsafe extern "C" fn f1(s: *mut ss) {
    let s = &mut *s;
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(&mut *S1.lock().unwrap());
    f1(&mut *S2.lock().unwrap());
    f1(arg as *mut ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s3 = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });
    let s3_ptr = Arc::into_raw(s3);

    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;

    pthread_create(
        &mut id1 as *mut pthread_t,
        ptr::null(),
        Some(t_fun),
        s3_ptr as *mut libc::c_void,
    );
    pthread_create(
        &mut id2 as *mut pthread_t,
        ptr::null(),
        Some(t_fun),
        s3_ptr as *mut libc::c_void,
    );

    pthread_join(id1, ptr::null_mut());
    pthread_join(id2, ptr::null_mut());

    Arc::from_raw(s3_ptr);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
