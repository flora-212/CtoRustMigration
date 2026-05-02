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

    fn f1(&self) {
        let mut lock1 = self.mutexes[0].lock().unwrap();
        let mut lock2 = self.mutexes[1].lock().unwrap();
        self.values[0] += 1;
        self.values[1] += 1;
    }

    fn f2(&self) {
        let mut lock2 = self.mutexes[1].lock().unwrap();
        let mut lock3 = self.mutexes[2].lock().unwrap();
        self.values[1] += 1;
        self.values[2] += 1;
    }

    fn f3(&self) {
        let mut lock3 = self.mutexes[2].lock().unwrap();
        let mut lock1 = self.mutexes[0].lock().unwrap();
        self.values[2] += 1;
        self.values[0] += 1;
    }
}

fn main() {
    let mutex_array = Arc::new(Mutex::new(MutexArray::new()));

    let mut handles = vec![];

    for i in 0..3 {
        let mutex_array_clone = Arc::clone(&mutex_array);
        let handle = thread::spawn(move || {
            match i {
                0 => mutex_array_clone.lock().unwrap().f1(),
                1 => mutex_array_clone.lock().unwrap().f2(),
                2 => mutex_array_clone.lock().unwrap().f3(),
                _ => unreachable!(),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = mutex_array.lock().unwrap();
    println!("{} {} {}", result.values[0], result.values[1], result.values[2]);
}