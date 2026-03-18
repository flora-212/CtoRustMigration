use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Clone)]
pub struct ss {
    pub n1: i32,
    pub m1: Arc<Mutex<i32>>,
    pub n2: i32,
    pub m2: Arc<RwLock<i32>>,
    pub n3: i32,
    pub m3: Arc<Mutex<i32>>, // Using Mutex instead of Spinlock
}

fn f1(s: &ss) {
    *s.m1.lock().unwrap() += 1;
    *s.m2.write().unwrap() += 1;
    *s.m3.lock().unwrap() += 1;
}

fn t_fun(s: Arc<ss>) {
    f1(&s);
}

fn main_0() -> i32 {
    let s = Arc::new(ss {
        n1: 1,
        m1: Arc::new(Mutex::new(0)),
        n2: 2,
        m2: Arc::new(RwLock::new(0)),
        n3: 3,
        m3: Arc::new(Mutex::new(0)), // Using Mutex instead of Spinlock
    });

    let s_clone1 = Arc::clone(&s);
    let s_clone2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || t_fun(s_clone1));
    let handle2 = thread::spawn(move || t_fun(s_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
