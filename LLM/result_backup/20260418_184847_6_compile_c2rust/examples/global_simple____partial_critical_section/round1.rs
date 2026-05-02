use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    n2: i32,
    n3: i32,
    num_mutex: Mutex<()>,
}

fn f1(shared: Arc<SharedData>) {
    let mut x;
    {
        let mut lock = shared.num_mutex.lock().unwrap();
        x = shared.n3;
        shared.n1 += x;
        shared.n2 += x;
        shared.n3 += x;
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared = Arc::from_raw(arg as *const SharedData);
    f1(shared);
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let shared = Arc::new(SharedData {
        n1: 0,
        n2: 0,
        n3: 1,
        num_mutex: Mutex::new(()),
    });

    let shared_clone1 = Arc::clone(&shared);
    let shared_clone2 = Arc::clone(&shared);

    let id1 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_clone1) as *mut libc::c_void);
    });

    let id2 = thread::spawn(move || {
        t_fun(Arc::into_raw(shared_clone2) as *mut libc::c_void);
    });

    id1.join().unwrap();
    id2.join().unwrap();

    unsafe {
        libc::printf(
            b"%d %d %d\n\0".as_ptr() as *const libc::c_char,
            shared.n1,
            shared.n2,
            shared.n3,
        );
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}