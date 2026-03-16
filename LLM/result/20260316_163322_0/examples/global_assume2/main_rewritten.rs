use std::sync::{Arc, Mutex};
use std::thread;

static mut N1: i32 = 0;
static NUM_MUTEX: Mutex<()> = Mutex::new(());

fn inc() -> i32 {
    unsafe {
        N1 += 1;
        if N1 != 0 { N1 } else { N1 + 1 }
    }
}

fn f1() {
    let _guard = NUM_MUTEX.lock().unwrap();
    inc();
}

extern "C" fn t_fun(_arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    std::ptr::null_mut()
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

    0 as libc::c_int
}

fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
