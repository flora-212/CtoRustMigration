use loom::sync::{Arc, Mutex};
use loom::thread;

#[derive(Debug)]
struct PthreadMutex {
    mutex: Mutex<()>,
}

impl PthreadMutex {
    fn new() -> Self {
        PthreadMutex {
            mutex: Mutex::new(()),
        }
    }

    fn lock(&self) -> loom::sync::MutexGuard<'_, ()> {
        self.mutex.lock().unwrap()
    }
}

#[test]
fn test_deadlock_with_dynamic_allocation() {
    loom::model(|| {
        // Create mutexes and data locally instead of using lazy_static
        let mutexes = Arc::new([PthreadMutex::new(), PthreadMutex::new()]);
        let data = Arc::new(Mutex::new([0i32; 2]));

        // Thread 1: Lock order 0 -> 1
        let mutexes1 = mutexes.clone();
        let data1 = data.clone();
        let handle1 = thread::spawn(move || {
            let _l0 = mutexes1[0].lock();
            let _l1 = mutexes1[1].lock();
            
            let mut n1 = data1.lock().unwrap();
            n1[0] += 1;
        });

        // Thread 2: Lock order 1 -> 0 (potential deadlock!)
        let mutexes2 = mutexes.clone();
        let data2 = data.clone();
        let handle2 = thread::spawn(move || {
            let _l1 = mutexes2[1].lock();
            let _l0 = mutexes2[0].lock();
            
            let mut n2 = data2.lock().unwrap();
            n2[1] += 1;
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    });
}
