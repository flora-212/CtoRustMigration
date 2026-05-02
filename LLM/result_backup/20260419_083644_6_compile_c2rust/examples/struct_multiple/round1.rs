use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct St {
    n1: i32,
    num_mutex: Mutex<()>,
}

static S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(()),
});

static S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(()),
});

static S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(()),
});

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
    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);
    let s3 = Arc::clone(&S3);

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