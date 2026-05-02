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
    let mut guard = s.m.lock().unwrap();
    s.n += 1;
}

unsafe fn f1(s1: &Arc<Ss>, s2: &Arc<Ss>) {
    let _guard1 = s1.m.lock().unwrap();
    let _guard2 = s2.m.lock().unwrap();
    inc(s1);
    inc(s2);
}

unsafe fn f2(s1: &Arc<Ss>, s2: &Arc<Ss>) {
    let _guard2 = s2.m.lock().unwrap();
    let _guard1 = s1.m.lock().unwrap();
    inc(s1);
    inc(s2);
}

unsafe fn t_fun1(arg: *mut Args) -> *mut Args {
    let a = &*arg;
    f1(&a.s1, &a.s2);
    arg
}

unsafe fn t_fun2(arg: *mut Args) -> *mut Args {
    let a = &*arg;
    f2(&a.s1, &a.s2);
    arg
}

unsafe fn main_0() -> i32 {
    let s1 = Arc::new(Ss { n: 0, m: Mutex::new(()) });
    let s2 = Arc::new(Ss { n: 0, m: Mutex::new(()) });

    let a = Args { s1: s1.clone(), s2: s2.clone() };

    let id1 = thread::spawn(move || {
        t_fun1(&mut a as *mut Args);
    });

    let id2 = thread::spawn(move || {
        t_fun2(&mut a as *mut Args);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    println!("{} {}", s1.n, s2.n);

    0
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}