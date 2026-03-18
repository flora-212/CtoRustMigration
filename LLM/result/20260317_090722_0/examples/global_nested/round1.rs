use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static mut N2: i32 = 0;

fn f1(n1_mutex: &Mutex<i32>, n2_mutex: &Mutex<i32>) {
    let mut n1_lock = n1_mutex.lock().unwrap();
    *n1_lock += 1;
    drop(n1_lock);

    let mut n2_lock = n2_mutex.lock().unwrap();
    *n2_lock += 1;
    drop(n2_lock);

    n1_lock = n1_mutex.lock().unwrap();
    *n1_lock += 1;
    drop(n1_lock);

    n2_lock = n2_mutex.lock().unwrap();
    let n1_value = *n1_mutex.lock().unwrap();
    *n2_lock += n1_value;
    drop(n2_lock);

    n1_lock = n1_mutex.lock().unwrap();
    *n1_lock += 1;
    drop(n1_lock);

    n2_lock = n2_mutex.lock().unwrap();
    *n2_lock += 1;
    drop(n2_lock);

    n1_lock = n1_mutex.lock().unwrap();
    let n2_value = *n2_mutex.lock().unwrap();
    *n1_lock += n2_value;
    drop(n1_lock);

    n2_lock = n2_mutex.lock().unwrap();
    *n2_lock += 1;
    drop(n2_lock);
}

fn t_fun(n1_mutex: Arc<Mutex<i32>>, n2_mutex: Arc<Mutex<i32>>) {
    f1(&n1_mutex, &n2_mutex);
}

fn main() {
    let n1_mutex = Arc::new(Mutex::new(0));
    let n2_mutex = Arc::new(Mutex::new(0));

    let n1_mutex_clone = Arc::clone(&n1_mutex);
    let n2_mutex_clone = Arc::clone(&n2_mutex);
    let handle1 = thread::spawn(move || t_fun(n1_mutex_clone, n2_mutex_clone));

    let n1_mutex_clone = Arc::clone(&n1_mutex);
    let n2_mutex_clone = Arc::clone(&n2_mutex);
    let handle2 = thread::spawn(move || t_fun(n1_mutex_clone, n2_mutex_clone));

    handle1.join().unwrap();
    handle2.join().unwrap();
}
