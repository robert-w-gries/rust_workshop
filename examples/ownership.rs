fn simple_types() {
    // Ownership rules do not apply to values stored in stack
    let mut a = 10;
    let b = a;
    a = 20;
    let s1 = "hello";
    let s2 = s1;
    println!("a = {}",a);
    println!("b = {}",b);
    println!("s1 = {} ; s2 = {}", s1, s2);
}


// Ownership is moved/transferred on assignment
fn test_ownership_1() {
//  simple_types(); // Ownership does not apply here

    let s1 = String::from("Hello");
    let mut s2 = s1; // Ownership of s1 is moved to s2
    s2.push_str("world"); // Modifying s2

    println!("s2 = {}", s2); // No Error

    println!("s1 = {}", s1); // Error
}

fn test_make_copy(x: String) {
  println!("string is {}",x);
}

// Ownership is moved/transferred with argument passing 
fn test_ownership_2() {
 let s1 = String::from("Hello");
 test_make_copy(s1); 
 println!("after make_copy {}",s1);
}

fn main() {
  test_ownership_1();
  test_ownership_2();
}

