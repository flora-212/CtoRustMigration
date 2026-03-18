use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Copy, Clone)]
pub struct St {
    pub n1: i32,
    pub num_mutex: Arc<Mutex<i32>>,
}

static mut S1: St = St {
    n1: 0,
    num_mutex: Arc::new(Mutex::new(0)),
};

static mut S2: St = St {
    n1: 1,
    num_mutex: Arc::new(Mutex::new(0)),
};

static mut S3: St = St {
    n1: 2,
    num_mutex: Arc::new(Mutex::new(0)),
};

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

fn f1() {
    unsafe {
        f(&S1);
        f(&S2);
        f(&S3);
    }
}

fn t_fun() {
    f1();
}

fn main_0() -> i32 {
    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}