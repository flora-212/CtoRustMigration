use std::sync::{Arc, Mutex};
use std::thread;

fn f1(mutex: &Mutex<i32>) {
    let mut x = 0;
    let mut n = mutex.lock().unwrap();
    while *n < 10 {
        x = *n;
        drop(n);
        x += 1;
        n = mutex.lock().unwrap();
        *n = x;
    }
}

fn f2(mutex: &Mutex<i32>) {
    let mut n = mutex.lock().unwrap();
    while *n < 20 {
        if *n > 18 {
            drop(n);
            return;
        }
        *n += 1;
    }
}

fn t_fun(mutex: Arc<Mutex<i32>>) {
    f1(&mutex);
    f2(&mutex);
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..2 {
        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || t_fun(mutex_clone));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
