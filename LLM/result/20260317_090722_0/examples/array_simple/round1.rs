use std::sync::{Mutex, Arc};
use std::thread;

static mut N1: [i32; 5] = [0; 5];
static mut N2: [i32; 5] = [1, 2, 3, 4, 5];
static mut N3: [i32; 5] = [1; 5];

static NUM_MUTEX: [Mutex<()>; 5] = [
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
];

fn f1() {
    for i in 0..5 {
        let x = unsafe { N3[i] };
        let mut guard = NUM_MUTEX[i].lock().unwrap();
        unsafe {
            N1[i] += x;
            N2[i] += x;
        }
        drop(guard);
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
