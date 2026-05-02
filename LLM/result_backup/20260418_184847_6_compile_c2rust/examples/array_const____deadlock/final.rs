use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: [i32; 3],
    num_mutex: [Mutex<()>; 3],
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            n1: [0; 3],
            num_mutex: [Mutex::new(()), Mutex::new(()), Mutex::new(())],
        }
    }

    fn f1(&mut self) {
        let _guard1 = self.num_mutex[0].lock().unwrap();
        let _guard2 = self.num_mutex[1].lock().unwrap();
        self.n1[0] += 1;
        self.n1[1] += 1;
    }

    fn f2(&mut self) {
        let _guard1 = self.num_mutex[1].lock().unwrap();
        let _guard2 = self.num_mutex[2].lock().unwrap();
        self.n1[1] += 1;
        self.n1[2] += 1;
    }

    fn f3(&mut self) {
        let _guard1 = self.num_mutex[2].lock().unwrap();
        let _guard2 = self.num_mutex[0].lock().unwrap();
        self.n1[2] += 1;
        self.n1[0] += 1;
    }
}

fn t_fun(shared_data: Arc<Mutex<SharedData>>, arg: i32) {
    let mut data = shared_data.lock().unwrap();
    match arg {
        0 => data.f1(),
        1 => data.f2(),
        _ => data.f3(),
    }
}

fn main_0() -> i32 {
    let shared_data = Arc::new(Mutex::new(SharedData::new()));

    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || t_fun(data_clone, i));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = shared_data.lock().unwrap();
    println!("{} {} {}", data.n1[0], data.n1[1], data.n1[2]);

    0
}

fn main() {
    std::process::exit(main_0());
}
