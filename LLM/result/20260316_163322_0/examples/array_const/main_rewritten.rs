use std::sync::{Arc, Mutex};
use std::thread;

fn f1(mutexes: &[Arc<Mutex<i32>>], n: &mut [i32]) {
    for i in 0..3 {
        let mut lock = mutexes[i].lock().unwrap();
        n[i] += 1;
    }
}

fn t_fun(mutexes: Arc<[Arc<Mutex<i32>>]>, n: Arc<Mutex<[i32; 3]>>) {
    let mut n = n.lock().unwrap();
    f1(&mutexes, &mut n);
}

fn main() {
    let mutexes: [Arc<Mutex<i32>>; 3] = [
        Arc::new(Mutex::new(0)),
        Arc::new(Mutex::new(0)),
        Arc::new(Mutex::new(0)),
    ];
    let n = Arc::new(Mutex::new([0; 3]));

    let mutexes_clone1 = Arc::clone(&Arc::from(mutexes.clone()));
    let n_clone1 = Arc::clone(&n);
    let handle1 = thread::spawn(move || t_fun(mutexes_clone1, n_clone1));

    let mutexes_clone2 = Arc::clone(&Arc::from(mutexes.clone()));
    let n_clone2 = Arc::clone(&n);
    let handle2 = thread::spawn(move || t_fun(mutexes_clone2, n_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    let n = n.lock().unwrap();
    println!("{:?}", n);
}
