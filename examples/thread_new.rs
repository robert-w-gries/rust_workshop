use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a thread that prints 5 messages.
    // Store a handle to the thread so we can call `join()` later.
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("message {}", i);

            // Sleep will yield execution back to the main thread!
            // If main thread is not blocked, the main thread will complete
            // execution then kill this child thread.
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("About to wait for thread to finish");

    // Block main thread until spawned thread has completed exeuction.
    // Note: Comment this line to see what happens to spawned thread!
    handle.join().unwrap();

    println!("Thread has finished!");
}
