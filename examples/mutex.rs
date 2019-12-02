use std::sync::{Mutex, MutexGuard};

fn main() {
    // Protecting data behind a `Mutex` is as simple as `Mutex::new(data)`
    let data = 42;
    let mutex: Mutex<u32> = Mutex::new(data);
    println!("{:?}", mutex); // Mutex { data: 42 }

    // Note: We declare an inner scope here because locks are dropped when
    // they leave scope or when `drop()` is manually called.
    {
        // Locking a `Mutex` yields a `MutexGuard`, a smart pointer that
        // keeps `Mutex` locked as long as it's alive and provides ability
        // to mutate inner data.
        let mut data_guard: MutexGuard<u32> = mutex.lock().unwrap();
        *data_guard = 24;

        // We do not have access to inner data until the `Mutex` is unlocked.
        println!("{:?}", mutex); // Mutex { data: <locked> }

        // implicit drop(data_guard) => mutex unlocked
    }

    // We have access to the data once again! And we can see the value has been
    // successfully changed.
    println!("{:?}", mutex); // Mutex { data: 24 }
}
