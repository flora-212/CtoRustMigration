use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static mut S1: Option<Arc<Ss>> = None;
static mut S2: Option<Arc<Ss>> = None;
static INIT: Once = Once::new();

fn init_globals() {
    unsafe {
        S1 = Some(Arc::new(Ss {
            n: 0,
            m: Mutex::new(()),
        }));
        S2 = Some(Arc::new(Ss {
            n: 0,
            m: Mutex::new(()),
        }));
    }
}

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s1 = S1.as_ref().unwrap();
    let s2 = S2.as_ref().unwrap();
    f1(Arc::get_mut_unchecked(s1) as *mut Ss);
    f1(Arc::get_mut_unchecked(s2) as *mut Ss);
    f1(arg as *mut Ss);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s3 = Arc::new(Ss {
        n: 0,
        m: Mutex::new(()),
    });

    let s3_clone = Arc::clone(&s3);

    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(Arc::clone(&s3)) as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    let s1 = S1.as_ref().unwrap();
    let s2 = S2.as_ref().unwrap();
    libc::printf(
        b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
        s1.n,
        s2.n,
        s3.n,
    );

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}