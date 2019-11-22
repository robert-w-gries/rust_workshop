use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("message {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("About to wait for thread to finish");
    handle.join().unwrap();
    println!("Thread has finished!");
}
