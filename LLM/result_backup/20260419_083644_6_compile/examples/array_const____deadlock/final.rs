use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexArray {
    mutexes: [Mutex<()>; 3],
    values: [i32; 3],
}

impl MutexArray {
    fn new() -> Self {
        MutexArray {
            mutexes: [Mutex::new(()), Mutex::new(()), Mutex::new(())],
            values: [0; 3],
        }
    }

    fn f1(&mut self) {
        let _lock1 = self.mutexes[0].lock().unwrap();
        let _lock2 = self.mutexes[1].lock().unwrap();
        self.values[0] += 1;
        self.values[1] += 1;
    }

    fn f2(&mut self) {
        let _lock1 = self.mutexes[1].lock().unwrap();
        let _lock2 = self.mutexes[2].lock().unwrap();
        self.values[1] += 1;
        self.values[2] += 1;
    }

    fn f3(&mut self) {
        let _lock1 = self.mutexes[2].lock().unwrap();
        let _lock2 = self.mutexes[0].lock().unwrap();
        self.values[2] += 1;
        self.values[0] += 1;
    }
}

fn t_fun(mutex_array: Arc<Mutex<MutexArray>>, arg: i32) {
    match arg {
        0 => {
            let mut array = mutex_array.lock().unwrap();
            array.f1();
        }
        1 => {
            let mut array = mutex_array.lock().unwrap();
            array.f2();
        }
        _ => {
            let mut array = mutex_array.lock().unwrap();
            array.f3();
        }
    }
}

fn main_0() -> i32 {
    let mutex_array = Arc::new(Mutex::new(MutexArray::new()));

    let mut handles = vec![];

    for i in 0..3 {
        let mutex_array_clone = Arc::clone(&mutex_array);
        handles.push(thread::spawn(move || t_fun(mutex_array_clone, i)));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let values = mutex_array.lock().unwrap().values;
    println!("{} {} {}", values[0], values[1], values[2]);

    0
}

fn main() {
    std::process::exit(main_0());
}
