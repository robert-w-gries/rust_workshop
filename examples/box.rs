use std::boxed::Box;

#[derive(Debug)]
enum List {
    Node(u32, Box<List>),
    Nil,
}

// error: "recursive type `Node` has infinite size"
#[cfg(feature = "broken")]
struct Node(Node);

fn main() {
    let i = &1;
    let boxed = Box::new(1);
    assert_eq!(*i, *boxed);

    let list = List::Node(0, Box::new(List::Node(1, Box::new(List::Nil))));
    println!("List = {:?}", list);
}
