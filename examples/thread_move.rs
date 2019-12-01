use std::thread;

// Lifetime Error
// "closure may outlive the current function, but it borrows `vec` which is owned by the current function"
//
// The main thread owns `vec` and will free it when the main thread completes exeuction.
// Since the spawned thread may live longer than the main thread, we have a potential use after free!
// Enforcement of ownership + lifetime prevents this potential buggy code from compiling.
#[cfg(feature = "broken")]
fn lifetime_error() {
    let vec = vec![1, 2, 3];
    thread::spawn(|| {
        println!("{:?}", vec);
    });
}

fn main() {
    let vec = vec![1, 2, 3];

    // The `move` keyword is needed in order to move ownership of `vec` to
    // the spawned thread.
    //
    // Note: we are manually freeing `vec` by calling `drop()` on it, but it
    // would have been implicitly dropped after thread has completed.
    thread::spawn(move || {
        println!("Vec = {:?}", vec);
        drop(vec);
    }).join().unwrap();

    // Ownership Error: "borrow of moved value: `vec`"
    //
    // Ownership prevents usage of freed resource!
    // The spawned thread took ownership of vec from the main thread due to the `move` keyword
    #[cfg(feature = "broken")]
    println!("Can't print freed vec: {:?}", vec);
}
