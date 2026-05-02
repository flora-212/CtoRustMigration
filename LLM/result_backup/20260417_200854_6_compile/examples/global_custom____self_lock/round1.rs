use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn lock(shared: &Arc<SharedData>) {
    let _guard = shared.num_mutex.lock().unwrap();
}

fn unlock(_guard: std::sync::MutexGuard<i32>) {}

fn f1(shared: &Arc<SharedData>) {
    let guard = lock(shared);
    let mut data = shared.num_mutex.lock().unwrap();
    *data += 1;
    drop(guard);
}

fn lock2(shared: &Arc<SharedData>, n: i32) -> i32 {
    let guard = lock(shared);
    let mut data = shared.num_mutex.lock().unwrap();
    *data += n;
    *data
}

fn unlock2(shared: &Arc<SharedData>, n: i32) -> i32 {
    let mut data = shared.num_mutex.lock().unwrap();
    *data += n;
    let result = *data;
    drop(data);
    result
}

fn f2(shared: &Arc<SharedData>) -> i32 {
    let n2 = lock2(shared, 1);
    let mut data = shared.num_mutex.lock().unwrap();
    *data += 1;
    drop(data);
    unlock2(shared, 1)
}

fn t_fun(shared: Arc<SharedData>) {
    lock(&shared);
    f1(&shared);
    let _ = f2(&shared);
    unlock(shared.num_mutex.lock().unwrap());
}

fn main_0() -> i32 {
    let shared_data = Arc::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(0),
    });

    let shared_clone1 = Arc::clone(&shared_data);
    let shared_clone2 = Arc::clone(&shared_data);

    let handle1 = thread::spawn(move || t_fun(shared_clone1));
    let handle2 = thread::spawn(move || t_fun(shared_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", shared_data.num_mutex.lock().unwrap());

    0
}

fn main() {
    std::process::exit(main_0());
}