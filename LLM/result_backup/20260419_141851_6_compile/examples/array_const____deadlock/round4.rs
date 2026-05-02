use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexList {
    __prev: Option<Arc<Mutex<()>>>,
    __next: Option<Arc<Mutex<()>>>,
}

#[derive(Debug)]
struct MutexData {
    __lock: i32,
    __count: u32,
    __owner: i32,
    __nusers: u32,
    __kind: i32,
    __spins: i16,
    __elision: i16,
    __list: MutexList,
}

#[derive(Debug)]
struct PthreadMutex {
    __data: MutexData,
}

#[derive(Debug)]
struct PthreadAttr {
    __size: [u8; 56],
    __align: i64,
}

const PTHREAD_MUTEX_TIMED_NP: u32 = 0;

static NUM_MUTEX: [Arc<Mutex<()>>; 3] = [
    Arc::new(Mutex::new(())),
    Arc::new(Mutex::new(())),
    Arc::new(Mutex::new(())),
];

fn f1(n1: &Arc<Mutex<[i32; 3]>>) {
    let _guard1 = NUM_MUTEX[0].lock().unwrap();
    let _guard2 = NUM_MUTEX[1].lock().unwrap();
    let mut n1 = n1.lock().unwrap();
    n1[0] += 1;
    n1[1] += 1;
}

fn f2(n1: &Arc<Mutex<[i32; 3]>>) {
    let _guard1 = NUM_MUTEX[1].lock().unwrap();
    let _guard2 = NUM_MUTEX[2].lock().unwrap();
    let mut n1 = n1.lock().unwrap();
    n1[1] += 1;
    n1[2] += 1;
}

fn f3(n1: &Arc<Mutex<[i32; 3]>>) {
    let _guard1 = NUM_MUTEX[2].lock().unwrap();
    let _guard2 = NUM_MUTEX[0].lock().unwrap();
    let mut n1 = n1.lock().unwrap();
    n1[2] += 1;
    n1[0] += 1;
}

fn t_fun(arg: i32, n1: &Arc<Mutex<[i32; 3]>>) {
    if arg == 0 {
        f1(n1);
    } else if arg == 1 {
        f2(n1);
    } else {
        f3(n1);
    }
}

fn main_0() -> i32 {
    let n1 = Arc::new(Mutex::new([0; 3]));

    let handles: Vec<_> = (0..3).map(|i| {
        let n1_clone = Arc::clone(&n1);
        thread::spawn(move || {
            t_fun(i, &n1_clone);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let n1 = n1.lock().unwrap();
    println!("{} {} {}", n1[0], n1[1], n1[2]);

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}