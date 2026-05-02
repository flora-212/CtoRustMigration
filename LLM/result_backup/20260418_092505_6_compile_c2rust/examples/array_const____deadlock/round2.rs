use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexArray {
    mutexes: [Mutex<i32>; 3],
    data: [Mutex<i32>; 3],
}

impl MutexArray {
    fn new() -> Self {
        MutexArray {
            mutexes: [Mutex::new(0), Mutex::new(0), Mutex::new(0)],
            data: [Mutex::new(0), Mutex::new(0), Mutex::new(0)],
        }
    }

    fn f1(&self) {
        let mut lock1 = self.mutexes[0].lock().unwrap();
        let mut lock2 = self.mutexes[1].lock().unwrap();
        *self.data[0].lock().unwrap() += 1;
        *self.data[1].lock().unwrap() += 1;
    }

    fn f2(&self) {
        let mut lock2 = self.mutexes[1].lock().unwrap();
        let mut lock3 = self.mutexes[2].lock().unwrap();
        *self.data[1].lock().unwrap() += 1;
        *self.data[2].lock().unwrap() += 1;
    }

    fn f3(&self) {
        let mut lock3 = self.mutexes[2].lock().unwrap();
        let mut lock1 = self.mutexes[0].lock().unwrap();
        *self.data[2].lock().unwrap() += 1;
        *self.data[0].lock().unwrap() += 1;
    }
}

fn t_fun(mutex_array: Arc<MutexArray>, arg: i32) {
    match arg {
        0 => mutex_array.lock().unwrap().f1(),
        1 => mutex_array.lock().unwrap().f2(),
        2 => mutex_array.lock().unwrap().f3(),
        _ => {}
    }
}

fn main_0() -> i32 {
    let mutex_array = Arc::new(MutexArray::new());

    let mut handles = vec![];

    for i in 0..3 {
        let mutex_array_clone = Arc::clone(&mutex_array);
        let handle = thread::spawn(move || t_fun(mutex_array_clone, i));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = mutex_array.lock().unwrap();
    println!("{} {} {}", *result.data[0].lock().unwrap(), *result.data[1].lock().unwrap(), *result.data[2].lock().unwrap());

    0
}

fn main() {
    std::process::exit(main_0());
}