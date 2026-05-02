use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexArray {
    mutexes: [Mutex<i32>; 3],
    values: [Mutex<i32>; 3],
}

impl MutexArray {
    fn new() -> Self {
        MutexArray {
            mutexes: [Mutex::new(0), Mutex::new(0), Mutex::new(0)],
            values: [Mutex::new(0), Mutex::new(0), Mutex::new(0)],
        }
    }

    fn increment(&self, index: usize) {
        let mut lock = self.mutexes[index].lock().unwrap();
        let mut value = self.values[index].lock().unwrap();
        *value += 1;
    }

    fn print(&self) {
        let v0 = self.values[0].lock().unwrap();
        let v1 = self.values[1].lock().unwrap();
        let v2 = self.values[2].lock().unwrap();
        println!("{} {} {}", *v0, *v1, *v2);
    }
}

fn f1(mutex_array: Arc<MutexArray>) {
    mutex_array.increment(0);
    mutex_array.increment(1);
    mutex_array.increment(2);
}

fn t_fun(mutex_array: Arc<MutexArray>) {
    f1(mutex_array);
}

fn main() {
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

    mutex_array.print();
}
