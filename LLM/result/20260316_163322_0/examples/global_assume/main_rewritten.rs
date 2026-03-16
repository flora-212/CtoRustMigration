use ::libc;
use std::sync::{Arc, Mutex};
use std::thread;

extern "C" {
    fn pthread_create(
        __newthread: *mut libc::c_ulong,
        __attr: *const libc::c_void,
        __start_routine: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}

static mut n1: libc::c_int = 0;

fn inc() {
    unsafe {
        n1 += 1;
    }
}

fn f1(mutex: &Mutex<()>) {
    let _guard = mutex.lock().unwrap();
    inc();
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let mutex = arg as *mut Mutex<()>;
    f1(&*mutex);
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mutex = Arc::new(Mutex::new(()));
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        Arc::into_raw(mutex.clone()) as *mut libc::c_void,
    );

    pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        Arc::into_raw(mutex.clone()) as *mut libc::c_void,
    );

    pthread_join(id1, std::ptr::null_mut());
    pthread_join(id2, std::ptr::null_mut());

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
