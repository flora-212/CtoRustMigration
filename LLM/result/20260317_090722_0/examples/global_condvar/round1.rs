use ::libc;
use std::sync::{Arc, Condvar, Mutex};

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

static mut n1: libc::c_int = 0;
static mut n2: libc::c_int = 0;

static NUM_MUTEX: Mutex<libc::c_int> = Mutex::new(0);
static COND: Condvar = Condvar::new();

unsafe extern "C" fn f1() {
    let mut num_mutex = NUM_MUTEX.lock().unwrap();
    n1 += 1;
    if n1 == 1 {
        while n1 == 1 {
            num_mutex = COND.wait(num_mutex).unwrap();
        }
    } else {
        COND.notify_one();
    }

    n2 += 1;
    if n2 == 1 {
        while n2 == 1 {
            num_mutex = COND.wait(num_mutex).unwrap();
        }
    } else {
        COND.notify_all();
    }
}

unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;

    pthread_create(
        &mut id1 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );

    pthread_create(
        &mut id2 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );

    pthread_join(id1, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut libc::c_void as *mut *mut libc::c_void);

    0 as libc::c_int
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
