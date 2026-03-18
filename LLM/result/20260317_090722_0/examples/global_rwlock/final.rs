use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let n = Arc::new(RwLock::new(0));

    let n1 = Arc::clone(&n);
    let n2 = Arc::clone(&n);

    let handle1 = thread::spawn(move || {
        t_fun(&n1);
    });

    let handle2 = thread::spawn(move || {
        t_fun(&n2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn f1(n: &Arc<RwLock<i32>>) -> i32 {
    let n = n.read().unwrap();
    *n
}

fn f2(n: &Arc<RwLock<i32>>) {
    let mut n = n.write().unwrap();
    *n += 1;
}

fn t_fun(n: &Arc<RwLock<i32>>) {
    f1(n);
    f2(n);
}
