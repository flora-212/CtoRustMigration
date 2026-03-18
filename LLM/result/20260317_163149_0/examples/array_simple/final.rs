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
    unsafe {
        for i in 0..5 {
            let x = N3[i];
            let _guard = NUM_MUTEX[i].lock().unwrap();
            N1[i] += x;
            N2[i] += x;
        }
    }
}

fn t_fun() {
    f1();
}

fn main() {
    let handle1 = thread::spawn(t_fun);
    let handle2 = thread::spawn(t_fun);

    handle1.join().unwrap();
    handle2.join().unwrap();
}
