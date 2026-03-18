use std::sync::{Arc, Mutex};
use std::thread;

fn inc(n: &Arc<Mutex<i32>>) {
    let mut num = n.lock().unwrap();
    *num += 1;
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
    let handle1 = thread::spawn(move || t_fun(n1));

    let n2 = Arc::clone(&n);
    let handle2 = thread::spawn(move || t_fun(n2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final value: {}", *n.lock().unwrap());
}
