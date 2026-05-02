use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct S {
    n: i32,
    m: Mutex<()>,
    c: std::sync::Condvar,
}

fn f1(s: Arc<Mutex<S>>) {
    let mut s = s.lock().unwrap();
    s.n += 1;
    if s.n == 1 {
        s.c.wait(&mut s);
    } else {
        s.c.notify_one();
    }
}

fn t_fun(s: Arc<Mutex<S>>) {
    f1(s);
}

fn main_0() -> i32 {
    let s = Arc::new(Mutex::new(S {
        n: 0,
        m: Mutex::new(()),
        c: std::sync::Condvar::new(),
    }));

    let s_clone1 = s.clone();
    let s_clone2 = s.clone();

    let handle1 = thread::spawn(move || t_fun(s_clone1));
    let handle2 = thread::spawn(move || t_fun(s_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let s = s.lock().unwrap();
    println!("{}", s.n);

    0
}

fn main() {
    std::process::exit(main_0());
}