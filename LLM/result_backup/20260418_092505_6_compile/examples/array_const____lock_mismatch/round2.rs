use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexArray {
    mutexes: [Mutex<i32>; 3],
    values: [i32; 3],
}

impl MutexArray {
    fn new() -> Self {
        MutexArray {
            mutexes: [Mutex::new(0), Mutex::new(0), Mutex::new(0)],
            values: [0; 3],
        }
    }

    fn increment(&self, index: usize) {
        let mut lock = self.mutexes[index].lock().unwrap();
        *lock += 1;
        self.values[index] = *lock;
    }
}

fn f1(mutex_array: Arc<MutexArray>) {
    mutex_array.increment(0);
    mutex_array.increment(1);
    mutex_array.increment(0);
}

fn t_fun(mutex_array: Arc<MutexArray>) {
    f1(mutex_array);
}

fn main_0() -> i32 {
    let mutex_array = Arc::new(MutexArray::new());

    let mut handles = vec![];

    for _ in 0..2 {
        let mutex_array_clone = Arc::clone(&mutex_array);
        let handle = thread::spawn(move || {
            t_fun(mutex_array_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = mutex_array.lock().unwrap();
    println!("{} {} {}", result.values[0], result.values[1], result.values[2]);

    0
}

fn main() {
    std::process::exit(main_0());
}