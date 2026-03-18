use std::sync::{Arc, Mutex};
use std::thread;

fn inc(n: &Arc<Mutex<i32>>) {
    let mut n = n.lock().unwrap();
    *n += 1;
}

fn f1(n: &Arc<Mutex<i32>>) {
    inc(n);
}

fn t_fun(n: Arc<Mutex<i32>>) {
    f1(&n);
}

fn main() {
    let n = Arc::new(Mutex::new(0));

    let n1 = Arc::clone(&n);
    let n2 = Arc::clone(&n);

    let handle1 = thread::spawn(move || t_fun(n1));
    let handle2 = thread::spawn(move || t_fun(n2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final value of n: {}", *n.lock().unwrap());
}