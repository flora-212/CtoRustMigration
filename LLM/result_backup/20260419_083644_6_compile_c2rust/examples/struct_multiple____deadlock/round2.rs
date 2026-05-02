use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct St {
    n1: i32,
    num_mutex: Mutex<i32>,
}

static S1: Arc<St> = Arc::new(St {
    n1: 0,
    num_mutex: Mutex::new(0),
});

static S2: Arc<St> = Arc::new(St {
    n1: 1,
    num_mutex: Mutex::new(1),
});

static S3: Arc<St> = Arc::new(St {
    n1: 2,
    num_mutex: Mutex::new(2),
});

fn f(s: &Arc<St>, t: &Arc<St>) {
    let mut s_guard = s.num_mutex.lock().unwrap();
    let mut t_guard = t.num_mutex.lock().unwrap();

    *s_guard = *t_guard + 1;
    *t_guard = *s_guard + 1;
}

fn f1() {
    f(&S1, &S2);
}

fn f2() {
    f(&S2, &S3);
}

fn f3() {
    f(&S1, &S3);
}

fn t_fun(arg: i32) {
    match arg {
        0 => f1(),
        1 => f2(),
        _ => f3(),
    }
}

fn main_0() -> i32 {
    let s1 = Arc::clone(&S1);
    let s2 = Arc::clone(&S2);
    let s3 = Arc::clone(&S3);

    let handle1 = thread::spawn(move || t_fun(0));
    let handle2 = thread::spawn(move || t_fun(1));
    let handle3 = thread::spawn(move || t_fun(2));

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    println!("{} {} {}", S1.n1, S2.n1, S3.n1);

    0
}

fn main() {
    std::process::exit(main_0());
}