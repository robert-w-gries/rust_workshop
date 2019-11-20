use std::sync::{Arc, Mutex};

fn main() {
    let ref_a: Arc<Mutex<u32>> = Arc::new(Mutex::new(42));
    {
        let ref_b: Arc<Mutex<u32>> = Arc::clone(&ref_a);
        drop(ref_a);
        *ref_b.lock().unwrap() = 24;
        println!("{:?}", ref_b);
    }
}
