use std::sync::{Arc, Mutex};
use std::thread;

pub struct ss {
    n: i32,
    m: Arc<Mutex<()>>,
}

fn main_0() -> i32 {
    let s1 = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });

    let s2 = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });

    let s3 = Arc::new(ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    });

    let s3_clone = s3.clone();
    let handle1 = thread::spawn(move || t_fun(&s3_clone, &s1, &s2));

    let s3_clone = s3.clone();
    let handle2 = thread::spawn(move || t_fun(&s3_clone, &s1, &s2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn t_fun(s3: &ss, s1: &ss, s2: &ss) {
    f1(s1);
    f1(s2);
    f1(s3);
}

fn f1(s: &ss) {
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

fn main() {
    std::process::exit(main_0());
}
