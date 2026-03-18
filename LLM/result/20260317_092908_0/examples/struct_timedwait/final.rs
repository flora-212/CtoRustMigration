use ::libc;
use std::sync::{Mutex, Condvar};
use std::time::{Duration, Instant};
use std::thread;

extern "C" {
    fn clock_gettime(__clock_id: libc::c_int, __tp: *mut timespec) -> libc::c_int;
}

pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

pub type clockid_t = __clockid_t;

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
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_456658959 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_554396209 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_554396208 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_554396209,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_456658959,
    pub __annonCompField2: __anonunion____missing_field_name_554396208,
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

#[derive(Debug)]
#[repr(C)]
pub struct __anonstruct_s_587009629 {
    pub n1: libc::c_int,
    pub n2: libc::c_int,
    pub n3: libc::c_int,
    pub m1: Mutex<()>,
    pub cond: Condvar,
}

static mut S: __anonstruct_s_587009629 = {
    let mut init = __anonstruct_s_587009629 {
        n1: 0 as libc::c_int,
        n2: 0 as libc::c_int,
        n3: 0 as libc::c_int,
        m1: Mutex::new(()),
        cond: Condvar::new(),
    };
    init
};

fn f1() {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut s = unsafe { &mut S };
    let mut guard = s.m1.lock().unwrap();
    s.n1 += 1;
    if s.n1 == 1 as libc::c_int {
        unsafe {
            clock_gettime(0 as libc::c_int, &mut ts);
        }
        ts.tv_sec += 1;
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        guard = s.cond.wait_until(guard, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f2() {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut s = unsafe { &mut S };
    let mut guard = s.m1.lock().unwrap();
    s.n2 += 1;
    if s.n2 == 1 as libc::c_int {
        unsafe {
            clock_gettime(0 as libc::c_int, &mut ts);
        }
        ts.tv_nsec += 1;
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        guard = s.cond.wait_until(guard, deadline).unwrap();
    } else {
        s.cond.notify_one();
    }
}

fn f3() {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut s = unsafe { &mut S };
    let mut guard = s.m1.lock().unwrap();
    s.n3 += 1;
    if s.n3 == 1 as libc::c_int {
        unsafe {
            clock_gettime(0 as libc::c_int, &mut ts);
        }
        ts.tv_sec += 1;
        ts.tv_nsec += 2 as libc::c_long;
        let deadline = Instant::now() + Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32);
        guard = s.cond.wait_until(guard, deadline).unwrap();
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

fn main_0() -> libc::c_int {
    let handle1 = thread::spawn(|| {
        unsafe {
            t_fun(0 as *mut libc::c_void);
        }
    });
    let handle2 = thread::spawn(|| {
        unsafe {
            t_fun(0 as *mut libc::c_void);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0 as libc::c_int
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
