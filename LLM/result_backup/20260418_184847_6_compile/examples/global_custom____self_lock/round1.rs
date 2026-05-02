use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

#[derive(Debug)]
struct SharedData {
    n1: i32,
    num_mutex: Mutex<()>,
}

impl SharedData {
    fn lock(&self) {
        self.num_mutex.lock().unwrap();
    }

    fn unlock(&self) {
        // Mutex guard is dropped here, automatically unlocking
    }

    fn f1(&mut self) {
        self.lock();
        self.n1 += 1;
        self.unlock();
    }

    fn lock2(&mut self, n: i32) -> i32 {
        self.lock();
        self.n1 += n;
        self.n1
    }

    fn unlock2(&mut self, n: i32) -> i32 {
        self.n1 += n;
        let n2 = self.n1;
        self.unlock();
        n2
    }

    fn f2(&mut self) -> i32 {
        let n2 = self.lock2(1);
        self.n1 += 1;
        let n2 = self.unlock2(1);
        n2
    }
}

unsafe extern "C" fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    let shared_data = Arc::from_raw(arg as *const SharedData);
    let mut shared_data = shared_data.lock().unwrap();
    shared_data.f1();
    shared_data.f2();
    ptr::null_mut()
}

fn main_0() -> libc::c_int {
    let shared_data = Arc::new(Mutex::new(SharedData {
        n1: 0,
        num_mutex: Mutex::new(()),
    }));

    let mut handles = vec![];

    for _ in 0..2 {
        let shared_data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            unsafe { t_fun(Arc::into_raw(shared_data_clone) as *mut libc::c_void) };
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let shared_data = Arc::try_unwrap(shared_data).unwrap().into_inner().unwrap();
    unsafe {
        libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, shared_data.n1);
    }

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}