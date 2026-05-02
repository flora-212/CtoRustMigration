use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct Ss {
    n: i32,
    m: Mutex<()>,
}

#[derive(Debug)]
struct Args {
    s1: Arc<Ss>,
    s2: Arc<Ss>,
}

unsafe fn inc(s: &Arc<Ss>) {
    let mut s = s.lock().unwrap();
    s.n += 1;
}

fn f1(s1: &Arc<Ss>, s2: &Arc<Ss>) {
    let _guard1 = s1.lock().unwrap();
    let _guard2 = s2.lock().unwrap();
    inc(s1);
    inc(s2);
}

fn f2(s1: &Arc<Ss>, s2: &Arc<Ss>) {
    let _guard2 = s2.lock().unwrap();
    let _guard1 = s1.lock().unwrap();
    inc(s1);
    inc(s2);
}

fn t_fun1(arg: &Args) {
    f1(&arg.s1, &arg.s2);
}

fn t_fun2(arg: &Args) {
    f2(&arg.s1, &arg.s2);
}

fn main_0() -> i32 {
    let s1 = Arc::new(Ss { n: 0, m: Mutex::new(()) });
    let s2 = Arc::new(Ss { n: 0, m: Mutex::new(()) });

    let args = Args { s1: s1.clone(), s2: s2.clone() };

    let handle1 = thread::spawn(move || t_fun1(&args));
    let handle2 = thread::spawn(move || t_fun2(&args));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} {}", s1.lock().unwrap().n, s2.lock().unwrap().n);

    0
}

fn main() {
    std::process::exit(main_0());
}