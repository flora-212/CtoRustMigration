use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static mut N2: i32 = 0;

fn f1(num_mutex: &Mutex<()>) {
    let _guard = num_mutex.lock().unwrap();
    unsafe {
        N1 += N2;
    }
}

fn t_fun(num_mutex: Arc<Mutex<()>>) {
    f1(&num_mutex);
}

fn main() {
    let num_mutex = Arc::new(Mutex::new(()));

    let mut handles = vec![];

    for _ in 0..2 {
        let num_mutex_clone = Arc::clone(&num_mutex);
        let handle = thread::spawn(move || {
            t_fun(num_mutex_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
