use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Ss {
    n1: i32,
    m1: Mutex<i32>,
    n2: i32,
    m2: Mutex<i32>,
    n3: i32,
    m3: Mutex<i32>,
}

fn f1(s: Arc<Ss>) {
    let mut n1 = s.m1.lock().unwrap();
    *n1 += 1;
    drop(n1);

    let mut n2 = s.m2.lock().unwrap();
    *n2 += 1;
    drop(n2);

    let mut n3 = s.m3.lock().unwrap();
    *n3 += 1;
    drop(n3);
}

fn t_fun(s: Arc<Ss>) {
    f1(s);
}

fn main_0() -> i32 {
    let s = Arc::new(Ss {
        n1: 1,
        m1: Mutex::new(0),
        n2: 2,
        m2: Mutex::new(0),
        n3: 3,
        m3: Mutex::new(0),
    });

    let s_clone1 = Arc::clone(&s);
    let s_clone2 = Arc::clone(&s);

    let handle1 = thread::spawn(move || t_fun(s_clone1));
    let handle2 = thread::spawn(move || t_fun(s_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", s.n1, s.n2, s.n3);

    0
}

fn main() {
    std::process::exit(main_0());
}
