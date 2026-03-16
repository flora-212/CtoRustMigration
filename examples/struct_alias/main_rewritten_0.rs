use std::sync::{Arc, Mutex};
use std::thread;

pub struct st {
    n1: i32,
    num_mutex: Arc<Mutex<()>>,
}

static mut S1: st = st {
    n1: 0,
    num_mutex: unsafe { Arc::new(Mutex::new(())) },
};

static mut S2: st = st {
    n1: 1,
    num_mutex: unsafe { Arc::new(Mutex::new(())) },
};

static mut S3: st = st {
    n1: 2,
    num_mutex: unsafe { Arc::new(Mutex::new(())) },
};

fn f(s: &mut st) {
    let _guard = s.num_mutex.lock().unwrap();
    s.n1 += 1;
    g(s);
}

fn g(t: &mut st) {
    t.n1 += 1;
    h(t);
}

fn h(u: &mut st) {
    u.n1 += 1;
}

fn f1() {
    unsafe {
        f(&mut S1);
        f(&mut S2);
        f(&mut S3);
    }
}

fn t_fun() {
    f1();
}

fn main() {
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(|| {
            t_fun();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
