use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::Once;

static mut S1: Option<Arc<St>> = None;
static mut S2: Option<Arc<St>> = None;
static mut S3: Option<Arc<St>> = None;
static INIT: Once = Once::new();

struct St {
    n1: i32,
    num_mutex: Mutex<()>,
}

fn initialize_globals() {
    unsafe {
        S1 = Some(Arc::new(St {
            n1: 0,
            num_mutex: Mutex::new(()),
        }));
        S2 = Some(Arc::new(St {
            n1: 1,
            num_mutex: Mutex::new(()),
        }));
        S3 = Some(Arc::new(St {
            n1: 2,
            num_mutex: Mutex::new(()),
        }));
    }
}

fn f(s: &Arc<St>) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
}

fn f1() {
    unsafe {
        if let Some(s1) = &S1 {
            f(s1);
        }
        if let Some(s2) = &S2 {
            f(s2);
        }
        if let Some(s3) = &S3 {
            f(s3);
        }
    }
}

fn t_fun() {
    f1();
}

fn main_0() -> i32 {
    INIT.call_once(initialize_globals);

    let s1 = unsafe { S1.as_ref().unwrap().clone() };
    let s2 = unsafe { S2.as_ref().unwrap().clone() };
    let s3 = unsafe { S3.as_ref().unwrap().clone() };

    let handle1 = thread::spawn(move || {
        t_fun();
    });

    let handle2 = thread::spawn(move || {
        t_fun();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("{} {} {}", S1.as_ref().unwrap().n1, S2.as_ref().unwrap().n1, S3.as_ref().unwrap().n1);
    }

    0
}

fn main() {
    std::process::exit(main_0());
}