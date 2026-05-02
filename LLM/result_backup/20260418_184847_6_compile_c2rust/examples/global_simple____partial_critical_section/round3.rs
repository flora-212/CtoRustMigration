use std::sync::{Arc, Mutex};
use std::thread;
use libc;

#[derive(Debug)]
struct SharedData {
    n1: Mutex<i32>,
    n2: Mutex<i32>,
    n3: Mutex<i32>,
    num_mutex: Mutex<()>,
}

fn f1(shared: Arc<SharedData>) {
    let mut x;
    {
        let mut lock = shared.num_mutex.lock().unwrap();
        x = *shared.n3.lock().unwrap();
        *shared.n1.lock().unwrap() += x;
        *shared.n2.lock().unwrap() += x;
        *shared.n3.lock().unwrap() += x;
    }
}

fn t_fun(shared: Arc<SharedData>) {
    f1(shared);
}

fn main_0() -> libc::c_int {
    let shared = Arc::new(SharedData {
        n1: Mutex::new(0),
        n2: Mutex::new(0),
        n3: Mutex::new(1),
        num_mutex: Mutex::new(()),
    });

    let shared_clone1 = Arc::clone(&shared);
    let shared_clone2 = Arc::clone(&shared);

    let id1 = thread::spawn(move || {
        t_fun(shared_clone1);
    });

    let id2 = thread::spawn(move || {
        t_fun(shared_clone2);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            *shared.n1.lock().unwrap(),
            *shared.n2.lock().unwrap(),
            *shared.n3.lock().unwrap(),
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}