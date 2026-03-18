use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Copy, Clone)]
pub struct ss0 {
    pub n1: i32,
    pub n2: i32,
}

#[derive(Clone)]
pub struct ss {
    pub s: Arc<Mutex<ss0>>,
    pub m: Arc<Mutex<()>>,
}

static S: ss = ss {
    s: Arc::new(Mutex::new(ss0 { n1: 0, n2: 1 })),
    m: Arc::new(Mutex::new(())),
};

fn f1(s: &ss) {
    let _guard = s.m.lock().unwrap();
    let mut data = s.s.lock().unwrap();
    data.n1 += 1;
    data.n2 += 1;
}

fn t_fun(s: Arc<ss>) {
    f1(&s);
}

fn main_0() -> i32 {
    let s = Arc::new(S.clone());
    let s2 = s.clone();

    let handle1 = thread::spawn(move || t_fun(s));
    let handle2 = thread::spawn(move || t_fun(s2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}