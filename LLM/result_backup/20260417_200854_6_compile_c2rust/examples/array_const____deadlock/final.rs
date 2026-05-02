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

    fn f1(&self) {
        let mut lock1 = self.mutexes[0].lock().unwrap();
        let mut lock2 = self.mutexes[1].lock().unwrap();
        let mut value0 = self.values[0].lock().unwrap();
        let mut value1 = self.values[1].lock().unwrap();
        *value0 += 1;
        *value1 += 1;
    }

    fn f2(&self) {
        let mut lock2 = self.mutexes[1].lock().unwrap();
        let mut lock3 = self.mutexes[2].lock().unwrap();
        let mut value1 = self.values[1].lock().unwrap();
        let mut value2 = self.values[2].lock().unwrap();
        *value1 += 1;
        *value2 += 1;
    }

    fn f3(&self) {
        let mut lock3 = self.mutexes[2].lock().unwrap();
        let mut lock1 = self.mutexes[0].lock().unwrap();
        let mut value2 = self.values[2].lock().unwrap();
        let mut value0 = self.values[0].lock().unwrap();
        *value2 += 1;
        *value0 += 1;
    }
}

fn main() {
    let mutex_array = Arc::new(MutexArray::new());

    let mut handles = vec![];

    for i in 0..3 {
        let mutex_array_clone = Arc::clone(&mutex_array);
        let handle = thread::spawn(move || {
            match i {
                0 => mutex_array_clone.f1(),
                1 => mutex_array_clone.f2(),
                2 => mutex_array_clone.f3(),
                _ => unreachable!(),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = mutex_array;
    println!("{} {} {}", result.values[0].lock().unwrap(), result.values[1].lock().unwrap(), result.values[2].lock().unwrap());
}
