use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Debug)]
struct AtomicWideCounter {
    value64: u64,
}

#[derive(Debug)]
struct PthreadMutex {
    lock: Mutex<()>,
    condvar: Condvar,
    count: u32,
    owner: Option<thread::ThreadId>,
    kind: i32,
}

impl PthreadMutex {
    fn new(kind: i32) -> Self {
        PthreadMutex {
            lock: Mutex::new(()),
            condvar: Condvar::new(),
            count: 0,
            owner: None,
            kind,
        }
    }

    fn lock(&self) {
        let mut guard = self.lock.lock().unwrap();
        while self.owner.is_some() && self.owner != Some(thread::current().id()) {
            guard = self.condvar.wait(guard).unwrap();
        }
        self.owner = Some(thread::current().id());
        self.count += 1;
    }

    fn unlock(&self) {
        let mut guard = self.lock.lock().unwrap();
        self.count -= 1;
        if self.count == 0 {
            self.owner = None;
        }
        self.condvar.notify_all();
    }
}

#[no_mangle]
static mut n1: i32 = 0;
#[no_mangle]
static mut n2: i32 = 0;
#[no_mangle]
static mut num_mutex: Arc<PthreadMutex> = Arc::new(PthreadMutex::new(0));
#[no_mangle]
static mut cond: Condvar = Condvar::new();

#[no_mangle]
unsafe extern "C" fn f1() {
    let num_mutex = Arc::clone(&num_mutex);
    let mut num_mutex = num_mutex.lock.lock().unwrap();
    n1 += 1;
    if n1 == 1 {
        num_mutex = num_mutex.condvar.wait(num_mutex).unwrap();
    } else {
        num_mutex = num_mutex.condvar.wait(num_mutex).unwrap();
    }
    drop(num_mutex);

    let num_mutex = Arc::clone(&num_mutex);
    let mut num_mutex = num_mutex.lock.lock().unwrap();
    n2 += 1;
    if n2 == 1 {
        num_mutex = num_mutex.condvar.wait(num_mutex).unwrap();
    } else {
        num_mutex.condvar.notify_all();
    }
    drop(num_mutex);
}

#[no_mangle]
unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let num_mutex = Arc::clone(&num_mutex);
    let cond = Arc::clone(&cond);

    let handle1 = thread::spawn(move || {
        f1();
    });

    let handle2 = thread::spawn(move || {
        f1();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    libc::printf(b"%d %d\n\0".as_ptr() as *const libc::c_char, n1, n2);
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}