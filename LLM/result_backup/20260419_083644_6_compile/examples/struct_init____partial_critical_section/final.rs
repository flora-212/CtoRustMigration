use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::sync::Once;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

static mut S1: Option<Arc<Mutex<Ss>>> = None;
static mut S2: Option<Arc<Mutex<Ss>>> = None;
static INIT: Once = Once::new();

fn init_globals() {
    unsafe {
        S1 = Some(Arc::new(Mutex::new(Ss {
            n: 0,
            m: Mutex::new(()),
        })));
        S2 = Some(Arc::new(Mutex::new(Ss {
            n: 0,
            m: Mutex::new(()),
        })));
    }
}

unsafe extern "C" fn f1(s: *mut Ss) {
    let s = &mut *s;
    s.n += 1;
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let s3 = arg as *mut Ss;
    let s1 = S1.as_ref().unwrap().clone();
    let s2 = S2.as_ref().unwrap().clone();
    f1(&mut *s1.lock().unwrap());
    f1(&mut *s2.lock().unwrap());
    f1(s3);
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s3 = Arc::new(Mutex::new(Ss {
        n: 0,
        m: Mutex::new(()),
    }));

    let s3_clone = Arc::clone(&s3);
    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    let s3_clone = Arc::clone(&s3);
    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(s3_clone) as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!(
        "{} {} {}",
        S1.as_ref().unwrap().lock().unwrap().n,
        S2.as_ref().unwrap().lock().unwrap().n,
        s3.lock().unwrap().n
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
