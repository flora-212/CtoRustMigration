use ::libc;
use std::ptr;
use std::sync::Mutex;

extern "C" {
    fn pthread_create(
        __newthread: *mut libc::c_ulong,
        __attr: *const libc::c_void,
        __start_routine: unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}

#[repr(C)]
pub struct ss {
    pub n: libc::c_int,
    pub m: Mutex<()>,
}

unsafe extern "C" fn inc(s: *mut ss) {
    (*s).n += 1;
}

unsafe extern "C" fn f1(s: *mut ss) {
    (*s).m.lock().unwrap();
    inc(s);
    drop((*s).m.lock().unwrap());
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(arg as *mut ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut s: *mut ss = ptr::null_mut();
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    s = libc::malloc(::std::mem::size_of::<ss>() as libc::size_t) as *mut ss;
    if s.is_null() {
        return -1;
    }

    (*s).n = 0;
    (*s).m = Mutex::new(());

    if libc::pthread_create(
        &mut id1,
        ptr::null(),
        t_fun,
        s as *mut libc::c_void,
    ) != 0
    {
        return -1;
    }

    if libc::pthread_create(
        &mut id2,
        ptr::null(),
        t_fun,
        s as *mut libc::c_void,
    ) != 0
    {
        return -1;
    }

    libc::pthread_join(id1, ptr::null_mut());
    libc::pthread_join(id2, ptr::null_mut());

    libc::free(s as *mut libc::c_void);

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
