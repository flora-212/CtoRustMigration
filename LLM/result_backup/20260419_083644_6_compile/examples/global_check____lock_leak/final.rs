use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexData {
    n: Mutex<i32>,
    m: Mutex<i32>,
}

impl MutexData {
    fn new() -> Self {
        MutexData {
            n: Mutex::new(0),
            m: Mutex::new(0),
        }
    }

    fn f1(&self) {
        let mut n = self.n.lock().unwrap();
        let mut m = self.m.lock().unwrap();
        *n += 1;
        *m += 1;
    }

    fn f2(&self) {
        let mut n = self.n.lock().unwrap();
        let mut m = self.m.lock().unwrap();
        *n += 1;
        *m += 1;
    }

    fn f3(&self) {
        let mut n = self.n.lock().unwrap();
        let mut m = self.m.lock().unwrap();
        *n += 1;
        *m += 1;
    }

    fn f4(&self) {
        let mut n = self.n.lock().unwrap();
        let mut m = self.m.lock().unwrap();
        *n += 1;
        *m += 1;
    }
}

fn t_fun(data: Arc<MutexData>) {
    data.f1();
    data.f2();
    data.f3();
    data.f4();
}

fn main_0() -> i32 {
    let data = Arc::new(MutexData::new());

    let handle1 = thread::spawn({
        let data_clone = Arc::clone(&data);
        move || {
            t_fun(data_clone);
        }
    });

    let handle2 = thread::spawn({
        let data_clone = Arc::clone(&data);
        move || {
            t_fun(data_clone);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = *data.n.lock().unwrap();
    println!("{}", n);

    0
}

fn main() {
    std::process::exit(main_0());
}
