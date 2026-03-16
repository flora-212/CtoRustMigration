use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Copy, Clone)]
pub struct ss0 {
    pub n1: i32,
    pub n2: i32,
}

#[derive(Clone)]
pub struct ss {
    pub s: Mutex<ss0>,
    pub m: Arc<Mutex<()>>,
}

fn main_0() -> i32 {
    let s = Arc::new(ss {
        s: Mutex::new(ss0 { n1: 0, n2: 1 }),
        m: Arc::new(Mutex::new(())),
    });

    let s1 = Arc::clone(&s);
    let handle1 = thread::spawn(move || {
        let _lock = s1.m.lock().unwrap();
        f1(&s1);
    });

    let s2 = Arc::clone(&s);
    let handle2 = thread::spawn(move || {
        let _lock = s2.m.lock().unwrap();
        f1(&s2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn f1(s: &ss) {
    let mut s = s.s.lock().unwrap();
    s.n1 += 1;
    s.n2 += 1;
}

fn main() {
    std::process::exit(main_0());
}
