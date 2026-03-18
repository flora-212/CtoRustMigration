use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Arc<Mutex<i32>>,
}

fn main_0() -> i32 {
    let s1 = St {
        n1: 0,
        num_mutex: Arc::new(Mutex::new(0)),
    };

    let s2 = St {
        n1: 1,
        num_mutex: Arc::new(Mutex::new(0)),
    };

    let s3 = St {
        n1: 2,
        num_mutex: Arc::new(Mutex::new(0)),
    };

    let handle1 = thread::spawn(move || {
        f(&s1);
        f(&s2);
        f(&s3);
    });

    let handle2 = thread::spawn(move || {
        f(&s1);
        f(&s2);
        f(&s3);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}

fn f(s: &St) {
    let mut num_mutex = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
    *num_mutex = 0; // Reset the mutex value if needed
}

fn g(t: &St) {
    t.n1 += 1;
    h(t);
}

fn h(u: &St) {
    u.n1 += 1;
}