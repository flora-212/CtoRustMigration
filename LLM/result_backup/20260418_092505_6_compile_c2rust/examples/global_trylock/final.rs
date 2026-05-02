use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexWrapper {
    mutex: Mutex<i32>,
}

impl MutexWrapper {
    fn new() -> Self {
        MutexWrapper {
            mutex: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut n = self.mutex.lock().unwrap();
        *n += 1;
    }
}

fn f1(mutex_wrapper: &MutexWrapper) {
    mutex_wrapper.increment();
    mutex_wrapper.increment();
    mutex_wrapper.increment();
    mutex_wrapper.increment();
}

fn t_fun(mutex_wrapper: Arc<MutexWrapper>) {
    f1(&mutex_wrapper);
}

fn main_0() -> i32 {
    let mutex_wrapper = Arc::new(MutexWrapper::new());

    let handle1 = thread::spawn({
        let mutex_wrapper = Arc::clone(&mutex_wrapper);
        move || t_fun(mutex_wrapper)
    });

    let handle2 = thread::spawn({
        let mutex_wrapper = Arc::clone(&mutex_wrapper);
        move || t_fun(mutex_wrapper)
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = mutex_wrapper.mutex.lock().unwrap();
    println!("{}", *n);

    0
}

fn main() {
    std::process::exit(main_0());
}
