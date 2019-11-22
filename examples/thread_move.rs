use std::thread;

fn main() {
    let vec = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vec = {:?}", vec);
        drop(vec);
    });

    handle.join().unwrap();

    // ownership error: "use of moved value: `vec`"
    #[cfg(feature = "broken")]
    println!("Can't print freed vec: {:?}", vec);
}
