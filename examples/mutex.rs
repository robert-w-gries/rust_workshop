use std::sync::{Mutex, MutexGuard};

fn main() {
    let locked_data: Mutex<u32> = Mutex::new(42);
    println!("{:?}", locked_data); // Mutex { data: 42 }
    {
        let mut data: MutexGuard<u32> = locked_data.lock().unwrap();
        *data = 24;
        println!("{:?}", locked_data); // Mutex { data: <locked> }
    }
    println!("{:?}", locked_data); // Mutex { data: 24 }
}
