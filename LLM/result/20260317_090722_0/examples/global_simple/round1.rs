use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static mut N2: i32 = 0;
static mut N3: i32 = 1;

fn f1(mutex: &Mutex<()>) {
    let x = unsafe { N3 };
    let mut guard = mutex.lock().unwrap();
    unsafe {
        N1 += x;
        N2 += x;
    }
}

fn t_fun(mutex: Arc<Mutex<()>>) {
    f1(&mutex);
}

fn main() {
    let mutex = Arc::new(Mutex::new(()));

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
        println!("N1: {}, N2: {}", N1, N2);
    }
}
