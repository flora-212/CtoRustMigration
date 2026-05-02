use std::sync::{Arc, Mutex};
use std::thread;
use libc;
use std::sync::OnceLock;

#[derive(Debug)]
struct Ss0 {
    n1: i32,
    n2: i32,
}

#[derive(Debug)]
struct Ss {
    s: Ss0,
    m: Mutex<()>,
}

static S: OnceLock<Arc<Mutex<Ss>>> = OnceLock::new();

fn f1(s: Arc<Mutex<Ss>>) {
    let mut s = s.lock().unwrap();
    s.s.n1 += 1;
    s.s.n2 += 1;
}

fn t_fun(s: Arc<Mutex<Ss>>) {
    f1(s);
}

fn main_0() -> libc::c_int {
    let s = S.get_or_init(|| Arc::new(Mutex::new(Ss {
        s: Ss0 { n1: 0, n2: 1 },
        m: Mutex::new(()),
    })));

    let handle1 = thread::spawn(move || {
        t_fun(s.clone());
    });

    let handle2 = thread::spawn(move || {
        t_fun(s.clone());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = s.lock().unwrap();
    unsafe {
        libc::printf(
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            s.s.n1,
            s.s.n2,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}