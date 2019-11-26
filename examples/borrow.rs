fn test_borrow() {
    let mut a = String::new();
    a.push_str("Hello");
    let mut b = a; // Move ownership of a to b
    println!("b = {}",b);
    {
        let  c = &mut b; // Take mutable reference of b
        c.push_str(" World!");
        println!("c = {}",c); // Hello World!
    }
    println!("b = {}",b); // Hello World!
}

fn main() {
  test_borrow();
}
