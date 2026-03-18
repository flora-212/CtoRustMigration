use std::sync::{Arc, Mutex};
use std::thread;

static mut N: i32 = 0;

fn f1(mutex: &Mutex<i32>) {
    let mut n = mutex.lock().unwrap();
    *n += 1;
    drop(n);

    let mut n = mutex.lock().unwrap();
    *n += 1;
    drop(n);

    let mut n = mutex.lock().unwrap();
    *n += 1;
    drop(n);

    let mut n = mutex.lock().unwrap();
    *n += 1;
    drop(n);
}

fn t_fun(mutex: Arc<Mutex<i32>>) {
    f1(&mutex);
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..2 {
        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            t_fun(mutex_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        println!("Final value of N: {}", N);
    }
}