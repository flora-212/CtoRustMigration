use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static mut N2: i32 = 0;
static NUM_MUTEX: Mutex<()> = Mutex::new(());

fn f1() {
    let _guard = NUM_MUTEX.lock().unwrap();
    unsafe {
        N1 += N2;
    }
}

fn t_fun() {
    f1();
}

fn main_0() -> i32 {
    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();

    0
}

fn main() {
    std::process::exit(main_0());
}
