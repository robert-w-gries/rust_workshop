use std::sync::{Arc, Mutex};
use std::thread;

// Ownership error: use of moved value: `counter`
//
// The mutex wrapper allows us to safely modify a shared reference in threaded
// contexts, but the mutex wrapper does not guarantee the shared reference will
// remain vaid across the different threads. Ownership saves us again from a
// potential usage after free!
#[cfg(feature = "broken")]
fn ownership_error() {
    let counter = Mutex::new(0);

    thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
        // `counter` is owned by this thread and is freed here
        // drop(counter) is implicitly called
    }).join().unwrap();

    thread::spawn(move || {
        // Error! The other thread has already freed the counter data.
        let mut num = counter.lock().unwrap();
        *num += 1;
    }).join().unwrap();
}

fn main() {
    // Our counter will be a simple u32 wrapped by a mutex.
    //
    // Note: `AtomicU32` would provide equivalent functionality without
    // needing expensive waits to acquire locks. But this example is
    // intended to show the common `Arc<Mutex<T>>` pattern.
    let counter: u32 = 0;

    // Getting a reference counted mutex to enable shared-state
    // concurrency is as easy as `Arc::new(Mutex::new(data))`!
    let counter_ref1 = Arc::new(Mutex::new(counter));
    let counter_ref2 = counter_ref1.clone();
    let counter_ref3 = counter_ref2.clone();

    // verify reference count == 3
    assert_eq!(Arc::strong_count(&counter_ref3), 3);

    // Increment counter by 1
    thread::spawn(move || {
        use std::ops::Deref;

        // dereference smart pointer `Arc` => `Mutex`
        let mutex = counter_ref1.deref();

        // lock mutex => `MutexGuard` smart pointer
        let mut data_guard = mutex.lock().unwrap(); 

        // dereference `MutexGuard` => mutable data
        *data_guard += 1;

        // implicit `drop(data_guard)` => mutex unlocked
        // implicit `drop(counter_ref1)` => reference count decremented
    }).join().unwrap();

    // Increment counter by 1
    thread::spawn(move || {
        *counter_ref2.lock().unwrap() += 1;
    }).join().unwrap();

    println!("Result: {}", *counter_ref3.lock().unwrap());
}
