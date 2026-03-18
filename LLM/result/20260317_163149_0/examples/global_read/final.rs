use std::sync::{Arc, Mutex};
use std::thread;

static mut n1: i32 = 0;
static mut n2: i32 = 0;
static NUM_MUTEX: Mutex<()> = Mutex::new(());

fn f1() {
    let _guard = NUM_MUTEX.lock().unwrap();
    unsafe {
        n1 += n2;
    }
}

fn t_fun() {
    f1();
}

fn main() {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(t_fun);
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
