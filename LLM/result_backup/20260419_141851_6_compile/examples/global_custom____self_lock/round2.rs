use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<i32>,
}

fn lock(shared: &Arc<SharedData>) -> std::sync::MutexGuard<i32> {
    shared.num_mutex.lock().unwrap()
}

fn unlock(_guard: std::sync::MutexGuard<i32>) {}

fn f1(shared: &Arc<SharedData>) {
    let mut guard = lock(shared);
    *guard += 1;
}

fn lock2(shared: &Arc<SharedData>, n: i32) -> i32 {
    let mut guard = lock(shared);
    *guard += n;
    *guard
}

fn unlock2(shared: &Arc<SharedData>, n: i32, guard: std::sync::MutexGuard<i32>) -> i32 {
    *guard += n;
    *guard
}

fn f2(shared: &Arc<SharedData>) -> i32 {
    let n2 = lock2(shared, 1);
    let mut guard = lock(shared);
    *guard += 1;
    unlock2(shared, 1, guard)
}

fn t_fun(shared: Arc<SharedData>) {
    let guard = lock(&shared);
    f1(&shared);
    let _ = f2(&shared);
    unlock(guard);
}

fn main_0() -> i32 {
    let shared = Arc::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(0),
    });

    let mut handles = vec![];

    for _ in 0..2 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || t_fun(shared_clone));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", shared.n1);
    0
}

fn main() {
    std::process::exit(main_0());
}