use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::ptr;

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
    kind: u32,
}

impl PthreadMutex {
    fn new(kind: u32) -> Self {
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
            self.condvar.notify_all();
        }
    }

    fn broadcast(&self) {
        self.condvar.notify_all();
    }

    fn wait(&self) {
        let mut guard = self.lock.lock().unwrap();
        while self.owner.is_some() && self.owner != Some(thread::current().id()) {
            guard = self.condvar.wait(guard).unwrap();
        }
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex::new(0);
static mut N1: i32 = 0;
static mut N2: i32 = 0;
static COND: Condvar = Condvar::new();

unsafe extern "C" fn f1() {
    NUM_MUTEX.lock();
    N1 += 1;
    if N1 == 1 {
        COND.wait(&mut NUM_MUTEX.lock());
    } else {
        COND.wait(&mut NUM_MUTEX.lock());
    }
    NUM_MUTEX.unlock();
    NUM_MUTEX.lock();
    N2 += 1;
    if N2 == 1 {
        COND.wait(&mut NUM_MUTEX.lock());
    } else {
        COND.notify_all();
    }
    NUM_MUTEX.unlock();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;
    libc::pthread_create(
        &mut id1,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        ptr::null(),
        Some(t_fun),
        ptr::null_mut(),
    );
    libc::pthread_join(id1, ptr::null_mut());
    libc::pthread_join(id2, ptr::null_mut());
    libc::printf(b"%d %d\n\0".as_ptr(), N1, N2);
    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}