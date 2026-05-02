use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    m1: Mutex<()>,
    m2: Mutex<()>,
}

static mut S: Option<Arc<SharedData>> = None;

fn init_shared_data() -> Arc<SharedData> {
    unsafe {
        if S.is_none() {
            S = Some(Arc::new(SharedData {
                n1: 0,
                n2: 1,
                n3: 2,
                n4: 3,
                m1: Mutex::new(()),
                m2: Mutex::new(()),
            }));
        }
        S.as_ref().unwrap().clone()
    }
}

fn f1(shared_data: Arc<SharedData>) {
    let x = shared_data.lock().unwrap().n4;
    let mut data = shared_data.lock().unwrap();
    data.n1 += x;
    data.n2 += x;
    drop(data);

    let mut data = shared_data.lock().unwrap();
    data.n3 += x;
}

fn t_fun(shared_data: Arc<SharedData>) {
    f1(shared_data);
}

fn main_0() -> i32 {
    let shared_data = init_shared_data();

    let handle1 = thread::spawn(move || t_fun(shared_data.clone()));
    let handle2 = thread::spawn(move || t_fun(shared_data.clone()));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = shared_data.lock().unwrap();
    println!("{} {} {} {}", data.n1, data.n2, data.n3, data.n4);

    0
}

fn main() {
    std::process::exit(main_0());
}