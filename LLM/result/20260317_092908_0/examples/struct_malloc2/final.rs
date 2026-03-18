use ::libc;
use std::ptr;
use std::sync::Mutex;

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

#[repr(C)]
struct ss {
    n: libc::c_int,
    m: Mutex<()>,
}

static mut x: *mut ss = ptr::null_mut();

unsafe extern "C" fn f1() {
    let s = Box::new(ss {
        n: 123,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);
    x = s_ptr;
    let _guard = (*s_ptr).m.lock().unwrap();
    (*s_ptr).n = 456;
}

unsafe extern "C" fn f2() {
    let s = Box::new(ss {
        n: 123,
        m: Mutex::new(()),
    });
    let s_ptr = Box::into_raw(s);
    x = s_ptr;
    let _guard = (*s_ptr).m.lock().unwrap();
    (*s_ptr).n = 789;
}

unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &mut id1 as *mut pthread_t,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );
    pthread_create(
        &mut id2 as *mut pthread_t,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );
    pthread_join(id1, ptr::null_mut());
    pthread_join(id2, ptr::null_mut());
    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
