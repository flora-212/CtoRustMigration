use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;
use std::sync::Lazy;

static S1: Lazy<Arc<St>> = Lazy::new(|| Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
}));

static S2: Lazy<Arc<St>> = Lazy::new(|| Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
}));

static S3: Lazy<Arc<St>> = Lazy::new(|| Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
}));

struct St {
    n1: i32,
    num_mutex: Mutex<()>,
}

fn f(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

fn f1() {
    f(&S1);
    f(&S2);
    f(&S3);
}

fn t_fun() {
    f1();
}

fn main_0() -> i32 {
    let s1 = S1.clone();
    let s2 = S2.clone();
    let s3 = S3.clone();

    let handle1 = thread::spawn(move || {
        t_fun();
    });

    let handle2 = thread::spawn(move || {
        t_fun();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", S1.n1, S2.n1, S3.n1);

    0
}

fn main() {
    std::process::exit(main_0());
}