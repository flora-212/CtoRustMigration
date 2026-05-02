use std::sync::{Arc, Mutex};
use std::thread;

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

fn inc(s: &Arc<Ss>) {
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

fn t_fun1(arg: &mut Args) {
    f1(&arg.s1, &arg.s2);
}

fn t_fun2(arg: &mut Args) {
    f2(&arg.s1, &arg.s2);
}

fn main_0() -> i32 {
    let s1 = Arc::new(Ss { n: 0, m: Mutex::new(()) });
    let s2 = Arc::new(Ss { n: 0, m: Mutex::new(()) });

    let mut a = Args { s1: s1.clone(), s2: s2.clone() };

    let id1 = thread::spawn(move || {
        t_fun1(&mut a);
    });

    let id2 = thread::spawn(move || {
        t_fun2(&mut a);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{} {}", s1.lock().unwrap().n, s2.lock().unwrap().n);

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}