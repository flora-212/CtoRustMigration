use std::sync::{Arc, Mutex};
use std::thread;

pub struct ss {
    n: i32,
    m: Arc<Mutex<()>>,
}

impl Clone for ss {
    fn clone(&self) -> Self {
        ss {
            n: self.n,
            m: self.m.clone(),
        }
    }
}

fn f1(s: &ss) {
    let _guard = s.m.lock().unwrap();
    s.n += 1;
}

fn t_fun(arg: &ss) {
    f1(&arg);
}

fn main_0() -> i32 {
    let s1 = ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    };

    let s2 = ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    };

    let s3 = ss {
        n: 0,
        m: Arc::new(Mutex::new(())),
    };

    let s3_clone = s3.clone();

    let handle1 = thread::spawn(move || {
        t_fun(&s1);
        t_fun(&s2);
        t_fun(&s3_clone);
    });

    let handle2 = thread::spawn(move || {
        t_fun(&s1);
        t_fun(&s2);
        t_fun(&s3_clone);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
