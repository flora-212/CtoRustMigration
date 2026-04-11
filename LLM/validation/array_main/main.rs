use loom::sync::{Arc, Mutex};
use loom::thread;
use std::ptr;
const N: usize = 5;

struct State {
    n1: Arc<Mutex<[i32; N]>>,
}

#[no_mangle]
fn f1_loom_wrapper(num_mutex: &loom::sync::Arc<[Arc<Mutex<i32>>; N]>) {
    let num_mutex = num_mutex;
    for i in 0..N {
        let mut num = num_mutex[i].lock().unwrap();
        *num += 1;
    }
}

#[test]
fn test_concurrent_access() {
    loom::model(|| {
        let state = loom::sync::Arc::new(State {
        n1: loom::sync::Arc::new(loom::sync::Mutex::new([0; N]))
        });
        let NUM_MUTEX = loom::sync::Arc::new([
            loom::sync::Arc::new(loom::sync::Mutex::new(0)),
            loom::sync::Arc::new(loom::sync::Mutex::new(0)),
            loom::sync::Arc::new(loom::sync::Mutex::new(0)),
            loom::sync::Arc::new(loom::sync::Mutex::new(0)),
            loom::sync::Arc::new(loom::sync::Mutex::new(0)),
        ]);
            for i in 0..N {
                state.n1.lock().unwrap()[i] += 1;
            }

            let mut handles = vec![];

            for _ in 0..2 {
                let num_mutex_clone = NUM_MUTEX.clone();
                let handle = loom::thread::spawn(move || {
                    unsafe {
                        f1_loom_wrapper(&num_mutex_clone);
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            for i in 0..N {
                state.n1.lock().unwrap()[i] += 1;
            }

    });
}