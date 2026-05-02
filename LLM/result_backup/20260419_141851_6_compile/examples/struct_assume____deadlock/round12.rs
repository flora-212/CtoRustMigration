use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use libc;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

#[derive(Debug)]
struct Args {
    s1: Arc<Ss>,
    s2: Arc<Ss>,
}

unsafe extern "C" fn inc(s: &mut Ss) {
    s.n += 1;
}

unsafe extern "C" fn f1(s1: &Arc<Ss>, s2: &Arc<Ss>) {
    let _guard1 = s1.m.lock().unwrap();
    let _guard2 = s2.m.lock().unwrap();
    inc(&mut s1.lock().unwrap());
    inc(&mut s2.lock().unwrap());
}

unsafe extern "C" fn f2(s1: &Arc<Ss>, s2: &Arc<Ss>) {
    let _guard2 = s2.m.lock().unwrap();
    let _guard1 = s1.m.lock().unwrap();
    inc(&mut s1.lock().unwrap());
    inc(&mut s2.lock().unwrap());
}

unsafe extern "C" fn t_fun1(arg: *mut libc::c_void) -> *mut libc::c_void {
    let a: *mut Args = arg as *mut Args;
    let args = &*a;
    f1(&args.s1, &args.s2);
    ptr::null_mut()
}

unsafe extern "C" fn t_fun2(arg: *mut libc::c_void) -> *mut libc::c_void {
    let a: *mut Args = arg as *mut Args;
    let args = &*a;
    f2(&args.s1, &args.s2);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let s1 = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });
    let s2 = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });

    let a = Args { s1: s1.clone(), s2: s2.clone() };

    let id1 = thread::spawn(move || {
        t_fun1(&mut a as *mut Args as *mut libc::c_void);
    });

    let id2 = thread::spawn(move || {
        t_fun2(&mut a as *mut Args as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    libc::printf(
        b"%d %d\n\0".as_ptr() as *const libc::c_char,
        s1.lock().unwrap().n,
        s2.lock().unwrap().n,
    );

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}