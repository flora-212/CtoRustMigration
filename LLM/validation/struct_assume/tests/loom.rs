use loom::sync::{Arc, Mutex};
use loom::thread;
use std::ptr;

#[repr(C)]
pub struct ss {
    pub n: i32,
    pub m: Mutex<()>,
}

#[no_mangle]
pub extern "C" fn inc(s: *mut ss) {
    unsafe {
        (*s).n += 1;
    }
}

#[no_mangle]
pub extern "C" fn f1(s: *mut ss) {
    unsafe {
        let mutex = &(*s).m;
        let _guard = mutex.lock().unwrap();
        inc(s);
    }
}

#[test]
fn test_concurrent_access() {
    loom::model(|| {
        let s = loom::sync::Arc::new(ss {
                n: 0,
                m: loom::sync::Mutex::new(()),
            });

            let mut handles = vec![];

            for _ in 0..2 {
                let s_clone = Arc::clone(&s);
                let handle = loom::thread::spawn(move || {
                    let s_ptr = Arc::into_raw(s_clone);
                    unsafe {
                        f1(s_ptr as *mut ss);
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            unsafe {
                let result = (*Arc::as_ptr(&s)).n;
                println!("{}", result);
            }
    });
}