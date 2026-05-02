use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    num_mutex: Mutex<()>,
    cond: Condvar,
}

fn f1(shared: Arc<SharedData>) {
    let mut n1 = shared.n1.lock().unwrap();
    let mut n2 = shared.n2.lock().unwrap();

    *n2 += 1;
    *n1 += 1;

    if *n1 == 1 {
        while *n1 == 1 {
            n1 = shared.cond.wait(n1).unwrap();
        }
    } else {
        shared.cond.notify_one();
    }

    *n2 += 1;

    if *n2 == 1 {
        while *n2 == 1 {
            n2 = shared.cond.wait(n2).unwrap();
        }
    } else {
        shared.cond.notify_all();
    }

    *n1 += 1;
}

fn t_fun(shared: Arc<SharedData>) {
    f1(shared);
}

fn main_0() -> i32 {
    let shared = Arc::new(SharedData {
        n1: Mutex::new(0),
        n2: Mutex::new(0),
        num_mutex: Mutex::new(()),
        cond: Condvar::new(),
    });

    let shared_clone1 = Arc::clone(&shared);
    let shared_clone2 = Arc::clone(&shared);

    let handle1 = thread::spawn(move || t_fun(shared_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n1 = *shared.n1.lock().unwrap();
    let n2 = *shared.n2.lock().unwrap();

    println!("{} {}", n1, n2);

    0
}

fn main() {
    std::process::exit(main_0());
}