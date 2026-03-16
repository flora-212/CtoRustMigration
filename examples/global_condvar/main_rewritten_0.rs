use std::sync::{Arc, Condvar, Mutex};
use std::thread;

static mut N1: i32 = 0;
static mut N2: i32 = 0;

fn f1(mutex: &Mutex<i32>, cond: &Condvar) {
    let mut num = mutex.lock().unwrap();
    unsafe {
        N1 += 1;
    }
    if unsafe { N1 } == 1 {
        while unsafe { N1 } == 1 {
            num = cond.wait(num).unwrap();
        }
    } else {
        cond.notify_one();
    }

    unsafe {
        N2 += 1;
    }
    if unsafe { N2 } == 1 {
        while unsafe { N2 } == 1 {
            num = cond.wait(num).unwrap();
        }
    } else {
        cond.notify_all();
    }
}

fn t_fun(mutex: Arc<Mutex<i32>>, cond: Arc<Condvar>) {
    f1(&mutex, &cond);
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let cond = Arc::new(Condvar::new());

    let mut threads = vec![];

    for _ in 0..2 {
        let mutex_clone = Arc::clone(&mutex);
        let cond_clone = Arc::clone(&cond);
        let thread = thread::spawn(move || t_fun(mutex_clone, cond_clone));
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
