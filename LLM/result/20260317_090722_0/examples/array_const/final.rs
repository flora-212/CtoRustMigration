use std::sync::{Mutex, Once};
use std::thread;
use libc::{pthread_t, pthread_create, pthread_join, pthread_mutex_init, pthread_attr_t};

static mut N1: [i32; 3] = [0; 3];
static NUM_MUTEX: [Mutex<i32>; 3] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];

static INIT: Once = Once::new();

unsafe extern "C" fn f1() {
    *NUM_MUTEX[0].lock().unwrap() += 1;
    *NUM_MUTEX[1].lock().unwrap() += 1;
    *NUM_MUTEX[2].lock().unwrap() += 1;
}

unsafe extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
}

unsafe fn main_0() -> libc::c_int {
    INIT.call_once(|| {
        for mutex in NUM_MUTEX.iter() {
            pthread_mutex_init(mutex as *const _ as *mut _, std::ptr::null());
        }
    });

    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;

    pthread_create(
        &mut id1,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );
    pthread_create(
        &mut id2,
        std::ptr::null(),
        Some(t_fun),
        std::ptr::null_mut(),
    );

    pthread_join(id1, std::ptr::null_mut());
    pthread_join(id2, std::ptr::null_mut());

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}
