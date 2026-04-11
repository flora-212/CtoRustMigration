use loom::sync::{Arc, Mutex};
use loom::thread;
use std::ptr;

#[no_mangle]
unsafe extern "C" fn inc(NUM_MUTEX: &Mutex<i32>) {
    let mut n1 = NUM_MUTEX.lock().unwrap();
    *n1 += 1;
}

#[no_mangle]
unsafe extern "C" fn f1(NUM_MUTEX: &Mutex<i32>) {
    inc(&NUM_MUTEX);
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut libc::c_void, NUM_MUTEX: &Mutex<i32>) -> *mut libc::c_void {
    f1(&NUM_MUTEX);
    ptr::null_mut()
}

struct State {
    N1: Arc<Mutex<i32>>,
    NUM_MUTEX: Mutex<i32>,
}

#[test]
fn test_concurrent_access() {
    loom::model(|| {
        let state = loom::sync::Arc::new(State {
        N1: loom::sync::Arc::new(loom::sync::Mutex::new(0)),
        NUM_MUTEX: loom::sync::Mutex::new(0)
        });
        let mut handles = vec![];

            for _ in 0..2 {
                let state_clone = state.clone();
                let handle = loom::thread::spawn(move || {
                    unsafe {
                        unsafe { t_fun(ptr::null_mut(), &state_clone.NUM_MUTEX) };
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            unsafe {
            }
    });
}