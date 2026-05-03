use loom::sync::{Arc, Mutex};
use loom::thread;
use std::ptr;
use std::ffi::CStr;

#[derive(Debug)]
struct Ss1 {
    n1: i32,
    n2: i32,
    m1: Mutex<i32>,
}

#[derive(Debug)]
struct Ss2 {
    n1: i32,
    n3: i32,
    m2: Mutex<i32>,
}

fn init_globals(state: &State) {
    unsafe {
        *state.S1.lock().unwrap()= Some(loom::sync::Arc::new(Ss1 {
            n1: 0,
            n2: 1,
            m1: loom::sync::Mutex::new(0),
        }));
        *state.S2.lock().unwrap()= Some(loom::sync::Arc::new(Ss2 {
            n1: 2,
            n3: 3,
            m2: loom::sync::Mutex::new(0),
        }));
    }
}

unsafe extern "C" fn f1(state: &State) {
    let x = S1.as_ref().unwrap().n2 + S2.as_ref().unwrap().n3;
    let mut guard1 = S1.as_ref().unwrap().m1.lock().unwrap();
    let mut guard2 = S2.as_ref().unwrap().m2.lock().unwrap();
    *guard1 += x;
    *guard2 += x;
}
fn main_0() -> libc::c_int {
    INIT.call_once(init_globals);

    let s1 = Arc::clone(unsafe { S1.as_ref().unwrap() });
    let s2 = Arc::clone(unsafe { S2.as_ref().unwrap() });

    let handle1 = loom::thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    let handle2 = loom::thread::spawn(move || {
        unsafe { t_fun(ptr::null_mut()) };
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            S1.as_ref().unwrap().n1,
            S1.as_ref().unwrap().n2,
        );
        libc::printf(
            b"%d %d\n\0".as_ptr() as *const libc::c_char,
            S2.as_ref().unwrap().n1,
            S2.as_ref().unwrap().n3,
        );
    }

    0
}

struct State {
    S1: Arc<Mutex<Option<Arc<Ss1>>>>,
    S2: Arc<Mutex<Option<Arc<Ss2>>>>,
    INIT: Once,
}

#[test]
fn test_concurrent_access() {
    loom::model(|| {
        let state = loom::sync::Arc::new(State {
        S1: loom::sync::Arc::new(loom::sync::Mutex::new(None)),
        S2: loom::sync::Arc::new(loom::sync::Mutex::new(None)),
        INIT: Once::new()
        });
        INIT.call_once(init_globals);

            let s1 = Arc::clone(unsafe { S1.as_ref().unwrap() });
            let s2 = Arc::clone(unsafe { S2.as_ref().unwrap() });

            let state1 = state.clone();
            let handle1 = loom::thread::spawn(move || {
                unsafe { f1(&state1) };
            });

            let state2 = state.clone();
            let handle2 = loom::thread::spawn(move || {
                unsafe {
                    unsafe { f1(&state2) };
                }
            });

            handle1.join().unwrap();
            handle2.join().unwrap();

            unsafe {
            }
    });
}
