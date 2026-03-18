use ::libc;
use std::sync::{Mutex, Condvar};
use std::time::{Duration, Instant};

extern "C" {
    fn clock_gettime(__clock_id: libc::c_int, __tp: *mut libc::timespec) -> libc::c_int;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::c_long,
    pub tv_nsec: libc::c_long,
}

pub type clockid_t = libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}

pub type __pthread_list_t = __pthread_internal_list;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: libc::c_ulonglong,
    pub __annonCompField2: libc::c_ulonglong,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}

pub type pthread_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}

pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_cond_t_951761805 {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}

pub type pthread_cond_t = __anonunion_pthread_cond_t_951761805;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_s_587009629 {
    pub n1: libc::c_int,
    pub n2: libc::c_int,
    pub n3: libc::c_int,
    pub m1: Mutex<libc::c_int>,
    pub cond: Condvar,
}

static mut S: __anonstruct_s_587009629 = {
    let mut init = __anonstruct_s_587009629 {
        n1: 0,
        n2: 0,
        n3: 0,
        m1: Mutex::new(0),
        cond: Condvar::new(),
    };
    init
};

fn f1() {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut s = unsafe { &mut S };
    let mut m1 = s.m1.lock().unwrap();
    s.n1 += 1;
    if s.n1 == 1 {
        unsafe {
            clock_gettime(0, &mut ts);
        }
        ts.tv_sec += 1;
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        m1 = s.cond.wait_until(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f2() {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut s = unsafe { &mut S };
    let mut m1 = s.m1.lock().unwrap();
    s.n2 += 1;
    if s.n2 == 1 {
        unsafe {
            clock_gettime(0, &mut ts);
        }
        ts.tv_nsec += 1;
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        m1 = s.cond.wait_until(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f3() {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut s = unsafe { &mut S };
    let mut m1 = s.m1.lock().unwrap();
    s.n3 += 1;
    if s.n3 == 1 {
        unsafe {
            clock_gettime(0, &mut ts);
        }
        ts.tv_sec += 1;
        ts.tv_nsec += 2;
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        m1 = s.cond.wait_until(m1, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    f3();
    0 as *mut libc::c_void
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &mut id1,
        0 as *const libc::c_void,
        t_fun,
        0 as *mut libc::c_void,
    );
    pthread_create(
        &mut id2,
        0 as *const libc::c_void,
        t_fun,
        0 as *mut libc::c_void,
    );
    pthread_join(id1, 0 as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut *mut libc::c_void);
    0 as libc::c_int
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
