use std::thread;

// Lifetime Error
// "closure may outlive the current function, but it borrows `vec` which is owned by the current function"
//
// The main thread owns `vec` and will free it when the thread is finished.
// Since the spawned thread may live longer than the main thread, we have a potentail use after
// free!
#[cfg(feature = "broken")]
fn broken_thread() {
    let vec = vec![1, 2, 3];
    thread::spawn(|| {
        println!("{:?}", vec);
    });
}

fn main() {
    let vec = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vec = {:?}", vec);
        drop(vec);
    });

    handle.join().unwrap();

    // Ownership Error: "use of moved value: `vec`"
    // Ownership prevents usage of free'd resource!
    // The spawned thread took ownership of vec from the main thread due to the `move` keyword
    #[cfg(feature = "broken")]
    println!("Can't print freed vec: {:?}", vec);
}
