
fn simple_types() {
    // Ownership rules do not apply to values stored in stack
    let mut a = 10;
    let b = a;
    a = 20;
    let s1 = "hello";
    let s2 = s1;
    println!("a = {}",a);
    println!("b = {}",b);
    println!("s2 = {}",s2);
}

fn test_ownership() {
    simple_types(); // Ownership does not apply here
    let s1= String::from("Hello");
    let mut s2 = s1; // Ownership of s1 is moved to s2
    s2.push_str("world"); // Modifying s2

    println!("s2 = {}", s2); // No Error

    #[cfg(feature = "broken")]
    println!("s1 = {}, s2 = {}", s1, s2); // Error
}

fn main() {

  test_ownership();

}

