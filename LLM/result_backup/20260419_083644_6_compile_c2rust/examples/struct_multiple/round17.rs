use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;
use std::sync::Lazy;

static S1: Lazy<Arc<St>> = Lazy::new(|| Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(0),
}));

static S2: Lazy<Arc<St>> = Lazy::new(|| Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(0),
}));

static S3: Lazy<Arc<St>> = Lazy::new(|| Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(0),
}));

struct St {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn f(s: &Arc<St>) {
    let mut num_mutex = s.num_mutex.lock().unwrap();
    *num_mutex += 1;
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
    let handle1 = thread::spawn(|| {
        t_fun();
    });

    let handle2 = thread::spawn(|| {
        t_fun();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {} {}", S1.num_mutex.lock().unwrap(), S2.num_mutex.lock().unwrap(), S3.num_mutex.lock().unwrap());

    0
}

fn main() {
    std::process::exit(main_0());
}