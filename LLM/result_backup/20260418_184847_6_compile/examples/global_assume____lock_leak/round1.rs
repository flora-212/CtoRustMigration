use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    pub fn lock(&self) {
        self.mutex.lock().unwrap();
    }
}

static NUM_MUTEX: PthreadMutex = PthreadMutex {
    mutex: Mutex::new(()),
};

static mut N1: i32 = 0;

unsafe extern "C" fn inc() {
    N1 += 1;
}

unsafe extern "C" fn f1() {
    NUM_MUTEX.lock();
    inc();
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;

    libc::pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );
    libc::pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );

    libc::pthread_join(id1, std::ptr::null_mut());
    libc::pthread_join(id2, std::ptr::null_mut());

    libc::printf(b"%d\n\0".as_ptr() as *const libc::c_char, N1);

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}