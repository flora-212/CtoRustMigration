use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexData {
    n: i32,
    m: Mutex<i32>,
}

impl MutexData {
    fn new() -> Self {
        MutexData {
            n: 0,
            m: Mutex::new(0),
        }
    }

    fn f1(&self) {
        let mut m = self.m.lock().unwrap();
        self.n += 1;
        *m += 1;
    }

    fn f2(&self) {
        let mut m = self.m.lock().unwrap();
        self.n += 1;
        *m += 1;
    }

    fn f3(&self) {
        let mut m = self.m.lock().unwrap();
        self.n += 1;
        *m += 1;
    }

    fn f4(&self) {
        let mut m = self.m.lock().unwrap();
        self.n += 1;
        *m += 1;
    }
}

fn t_fun(data: Arc<MutexData>) {
    let data_clone = Arc::clone(&data);
    data_clone.f1();
    data_clone.f2();
    data_clone.f3();
    data_clone.f4();
}

fn main_0() -> i32 {
    let data = Arc::new(MutexData::new());

    let handle1 = thread::spawn(move || {
        t_fun(Arc::clone(&data));
    });

    let handle2 = thread::spawn(move || {
        t_fun(Arc::clone(&data));
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = data.lock().unwrap().n;
    println!("{}", n);

    0
}

fn main() {
    std::process::exit(main_0());
}