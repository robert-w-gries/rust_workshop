use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(feature = "broken")]
fn broken_mutex() {
    let counter = Mutex::new(0);

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();
        *num2 += 1;
    });

    handle.join().unwrap();
    handle2.join().unwrap();

    println!("Result: {}", *counter.lock().unwrap());
}

fn working_mutex() {
    let counter_ref1 = Arc::new(Mutex::new(0));
    let counter_ref2 = Arc::clone(&counter_ref1);
    let counter_ref3 = Arc::clone(&counter_ref2);

    let handle = thread::spawn(move || {
        let mut num = counter_ref1.lock().unwrap();
        *num += 1;
    });

    let handle2 = thread::spawn(move || {
        let mut num2 = counter_ref2.lock().unwrap();
        *num2 += 1;
    });

    handle.join().unwrap();
    handle2.join().unwrap();

    println!("Result: {}", *counter_ref3.lock().unwrap());
}

fn main() {
    working_mutex();
}
