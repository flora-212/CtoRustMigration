use ::libc;
use std::sync::{Arc, Mutex};
use std::thread;

extern "C" {
    fn pthread_create(
        __newthread: *mut libc::c_ulong,
        __attr: *const libc::c_void,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}

static mut n1: libc::c_int = 0;

fn f1(mutex: &Mutex<libc::c_int>) {
    let mut n = mutex.lock().unwrap();
    *n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let mutex = Arc::from_raw(arg as *const Mutex<libc::c_int>);
    f1(&mutex);
    Arc::into_raw(mutex);
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mutex = Arc::new(Mutex::new(n1));
    let mutex_clone = Arc::clone(&mutex);

    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    n1 += 1;

    pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        Arc::into_raw(mutex_clone) as *mut libc::c_void,
    );

    let mutex_clone = Arc::clone(&mutex);

    pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        Arc::into_raw(mutex_clone) as *mut libc::c_void,
    );

    pthread_join(id1, std::ptr::null_mut());
    pthread_join(id2, std::ptr::null_mut());

    n1 += 1;

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
