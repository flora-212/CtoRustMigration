use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;

pub struct st {
    n1: i32,
    num_mutex: Arc<Mutex<()>>,
}

lazy_static! {
    static ref S1: Arc<Mutex<st>> = Arc::new(Mutex::new(st {
        n1: 0,
        num_mutex: Arc::new(Mutex::new(())),
    }));
    static ref S2: Arc<Mutex<st>> = Arc::new(Mutex::new(st {
        n1: 1,
        num_mutex: Arc::new(Mutex::new(())),
    }));
    static ref S3: Arc<Mutex<st>> = Arc::new(Mutex::new(st {
        n1: 2,
        num_mutex: Arc::new(Mutex::new(())),
    }));
}

fn f(s: &mut st) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
}

fn g(t: &mut st) {
    t.n1 += 1;
    h(t);
}

fn h(u: &mut st) {
    u.n1 += 1;
}

fn f1() {
    let mut s1 = S1.lock().unwrap();
    let mut s2 = S2.lock().unwrap();
    let mut s3 = S3.lock().unwrap();

    f(&mut s1);
    f(&mut s2);
    f(&mut s3);
}

fn t_fun() {
    f1();
}

fn main() {
    let handle1 = thread::spawn(|| t_fun());
    let handle2 = thread::spawn(|| t_fun());

    handle1.join().unwrap();
    handle2.join().unwrap();
}
