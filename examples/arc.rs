use std::sync::Arc;
use std::thread;

// error: `std::rc::Rc<std::vec::Vec<i32>>` cannot be shared between threads safely
//
// `Rc` is a type that only works in single-threaded contexts. To be specific, `Rc`
// does not implement the `Sync` trait which is required, along with the `Send` trait,
// for any value that is moved into a thread. Most types automatically implement `Send`
// and `Sync`, as long as the compiler can guarantee that they are thread-safe to share.
#[cfg(feature = "broken")]
fn single_threaded_rc() {
    use std::rc::Rc;

    let vec = vec![1, 2, 3];

    // Reference counted pointer to the `Vec` data.
    //
    // `Rc` is used to enable multiple ownership of data. A common use case is
    // a graph object that has edges referencing the same nodes.
    let ref_a = Rc::new(vec);
    let ref_b = ref_a.clone();

    // Error! `Rc` is only used usable in single-threaded contexts.
    thread::spawn(move || {
        println!("Vec = {:?}", ref_b);
    });
}

fn main() {
    let vec = vec![1, 2, 3];

    // `Arc` is a wrapper for our data. With this line, we are now
    // reference counting the data, and the data will be freed when
    // the reference count hits zero!
    //
    // Note: The difference between `Rc` and `Arc` is that `Arc` uses atomics
    // to ensure synchronization of the data. Remember that synchronization
    // prevents data races!
    let ref_a: Arc<Vec<u32>> = Arc::new(vec);

    // Cloning a reference increments the reference count.
    //
    // Note: Since `Arc` implements the `Clone` trait, we can use `ref.clone()`
    // method instead of `Arc::clone()`
    let ref_b: Arc<Vec<u32>> = Arc::clone(&ref_a);
    let ref_c: Arc<Vec<u32>> = ref_a.clone();

    // Verify reference count == 3
    assert_eq!(Arc::strong_count(&ref_a), 3);

    thread::spawn(move || {
        println!("Vec = {:?}", ref_b);
        // implicit drop(ref_b); referenece count decremented
    }).join().unwrap();

    assert_eq!(Arc::strong_count(&ref_a), 2);

    thread::spawn(move || {
        println!("Vec = {:?}", ref_c);

        // error: cannot borrow data in an `Arc` as mutable
        //
        // `Arc` provides shared ownership of data, but it does not provide
        // shared mutability. To mutate the reference provided by an Arc, we
        // need something called 'interior mutablility'. Interior mutability
        // allows an immutable object to mutate the data held by the object.
        // One struct that provides this capability is the `Mutex` struct.
        #[cfg(feature = "broken")]
        ref_c.push(4);

        // implicit drop(ref_c); referenece count decremented
    }).join().unwrap();

    assert_eq!(Arc::strong_count(&ref_a), 1);
}
