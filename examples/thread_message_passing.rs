use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || {
        tx1.send(String::from("Hello")).unwrap();
    });

    thread::spawn(move || {
        tx2.send(String::from("World")).unwrap();
    });

    for received in rx { // blocks until channel hangs up
        println!("Got: {}", received);
    }
}
