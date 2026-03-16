use ::libc;
use std::sync::{Arc, Mutex};
use std::thread;

extern "C" {
    fn pthread_create(
        __newthread: *mut libc::c_ulong,
        __attr: *const libc::c_void,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}

static mut n1: libc::c_int = 0;
static mut n2: libc::c_int = 0;
static NUM_MUTEX: Mutex<()> = Mutex::new(());

unsafe extern "C" fn f1() {
    let _guard = NUM_MUTEX.lock().unwrap();
    n1 += n2;
}

unsafe extern "C" fn t_fun(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );
    pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );

    pthread_join(id1, std::ptr::null_mut());
    pthread_join(id2, std::ptr::null_mut());

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
