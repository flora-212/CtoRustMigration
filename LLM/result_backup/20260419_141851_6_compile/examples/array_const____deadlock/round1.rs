use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MutexList {
    __prev: Option<Arc<Mutex<()>>>,
    __next: Option<Arc<Mutex<()>>>,
}

#[derive(Debug)]
struct MutexData {
    __lock: i32,
    __count: u32,
    __owner: i32,
    __nusers: u32,
    __kind: i32,
    __spins: i16,
    __elision: i16,
    __list: MutexList,
}

#[derive(Debug)]
struct PthreadMutex {
    __data: MutexData,
}

#[derive(Debug)]
struct PthreadAttr {
    __size: [u8; 56],
    __align: i64,
}

const PTHREAD_MUTEX_TIMED_NP: u32 = 0;

static mut N1: [i32; 3] = [0; 3];
static NUM_MUTEX: [Arc<Mutex<()>>; 3] = [
    Arc::new(Mutex::new(())),
    Arc::new(Mutex::new(())),
    Arc::new(Mutex::new(())),
];

fn f1() {
    let _guard1 = NUM_MUTEX[0].lock().unwrap();
    let _guard2 = NUM_MUTEX[1].lock().unwrap();
    unsafe {
        N1[0] += 1;
        N1[1] += 1;
    }
}

fn f2() {
    let _guard1 = NUM_MUTEX[1].lock().unwrap();
    let _guard2 = NUM_MUTEX[2].lock().unwrap();
    unsafe {
        N1[1] += 1;
        N1[2] += 1;
    }
}

fn f3() {
    let _guard1 = NUM_MUTEX[2].lock().unwrap();
    let _guard2 = NUM_MUTEX[0].lock().unwrap();
    unsafe {
        N1[2] += 1;
        N1[0] += 1;
    }
}

fn t_fun(arg: *mut libc::c_void) -> *mut libc::c_void {
    if arg as i64 == 0 {
        f1();
    } else if arg as i64 == 1 {
        f2();
    } else {
        f3();
    }
    libc::NULL
}

unsafe fn main_0() -> libc::c_int {
    let mut id1: libc::pthread_t = 0;
    let mut id2: libc::pthread_t = 0;
    let mut id3: libc::pthread_t = 0;

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
        1 as *mut libc::c_void,
    );
    libc::pthread_create(
        &mut id3,
        std::ptr::null(),
        Some(t_fun),
        2 as *mut libc::c_void,
    );

    libc::pthread_join(id1, std::ptr::null_mut());
    libc::pthread_join(id2, std::ptr::null_mut());
    libc::pthread_join(id3, std::ptr::null_mut());

    libc::printf(
        b"%d %d %d\n\0".as_ptr(),
        N1[0],
        N1[1],
        N1[2],
    );

    0
}

fn main() {
    unsafe { std::process::exit(main_0() as i32) }
}