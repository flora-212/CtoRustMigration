use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[derive(Debug)]
struct SharedState {
    n: Mutex<i32>,
    m: Mutex<()>,
    c: Condvar,
}

fn f1(s: Arc<SharedState>) {
    let mut m = s.m.lock().unwrap();
    let mut n = s.n.lock().unwrap();
    *n += 1;
    if *n == 1 {
        m = s.c.wait(m).unwrap();
    } else {
        s.c.notify_one();
    }
}

fn t_fun(s: Arc<SharedState>) {
    f1(s);
}

fn main() {
    let s = Arc::new(SharedState {
        n: Mutex::new(0),
        m: Mutex::new(()),
        c: Condvar::new(),
    });

    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || t_fun(s1));
    let handle2 = thread::spawn(move || t_fun(s2));

    handle1.join().unwrap();
    handle2.join().unwrap();
}
