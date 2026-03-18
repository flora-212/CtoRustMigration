use ::libc;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ss {
    pub n: libc::c_int,
    pub m: Mutex<()>,
}

pub static mut x: Option<Arc<ss>> = None;

fn f1() {
    let s = Arc::new(ss {
        n: 123,
        m: Mutex::new(()),
    });
    unsafe { x = Some(s.clone()) };
    let mut guard = s.m.lock().unwrap();
    s.n = 456;
    drop(guard);
}

fn f2() {
    let s = Arc::new(ss {
        n: 789,
        m: Mutex::new(()),
    });
    unsafe { x = Some(s.clone()) };
    let mut guard = s.m.lock().unwrap();
    s.n = 789;
    drop(guard);
}

fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::c_ulong = 0;
    let mut id2: libc::c_ulong = 0;

    let handle1 = thread::spawn(|| t_fun(ptr::null_mut()));
    let handle2 = thread::spawn(|| t_fun(ptr::null_mut()));

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}