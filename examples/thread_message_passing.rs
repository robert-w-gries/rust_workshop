use std::sync::mpsc;
use std::thread;

fn main() {
    // `channel()` returns a tuple of a `Sender` and a `Receiver`.
    // Note: In a single-consumer channel, we can only have one `Receiver`
    let (tx1, rx) = mpsc::channel();

    // Since it's a MULTI-producer channel, we can clone the senders.
    let tx2 = mpsc::Sender::clone(&tx1);
    let tx3 = tx2.clone();

    thread::spawn(move || {
        tx1.send(String::from("Hello")).unwrap();
        // thread completed => implicit drop(tx1)
    });

    thread::spawn(move || {
        tx2.send(String::from("World")).unwrap();
        // thread completed => implicit drop(tx2)
    });

    thread::spawn(move || {
        tx3.send(String::from("!")).unwrap();
        // thread completed => implicit drop(tx3)
    });

    // This loop blocks until the channel hangs up, meaning
    // that all `Sender`s have dropped or `Receiver` has dropped.
    for received in rx {
        println!("Got: {}", received);
    }
}
