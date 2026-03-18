use std::sync::{Arc, Mutex};
use std::thread;

pub struct st {
    n1: i32,
    num_mutex: Arc<Mutex<()>>,
}

fn main_0() -> i32 {
    let s1 = Arc::new(Mutex::new(st {
        n1: 0,
        num_mutex: Arc::new(Mutex::new(())),
    }));

    let s2 = Arc::new(Mutex::new(st {
        n1: 1,
        num_mutex: Arc::new(Mutex::new(())),
    }));

    let s3 = Arc::new(Mutex::new(st {
        n1: 2,
        num_mutex: Arc::new(Mutex::new(())),
    }));

    fn f(s: &mut st) {
        let _guard = s.num_mutex.lock().unwrap();
        s.n1 += 1;
    }

    fn f1(s1: &mut st, s2: &mut st, s3: &mut st) {
        f(s1);
        f(s2);
        f(s3);
    }

    fn t_fun(_arg: *mut libc::c_void, s1: Arc<Mutex<st>>, s2: Arc<Mutex<st>>, s3: Arc<Mutex<st>>) -> *mut libc::c_void {
        let mut s1 = s1.lock().unwrap();
        let mut s2 = s2.lock().unwrap();
        let mut s3 = s3.lock().unwrap();
        f1(&mut s1, &mut s2, &mut s3);
        0 as *mut libc::c_void
    }

    let mut handles = vec![];

    for _ in 0..2 {
        let s1_clone = s1.clone();
        let s2_clone = s2.clone();
        let s3_clone = s3.clone();
        let handle = thread::spawn(move || {
            unsafe { t_fun(0 as *mut libc::c_void, s1_clone, s2_clone, s3_clone) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    0
}

fn main() {
    std::process::exit(main_0() as i32);
}
