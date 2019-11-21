fn test_borrow() {
    let mut a = String::new();
    a.push_str("Hello");
    let mut b = a;
    println!("b = {}",b);
    {
        let  c = &mut b;
        println!("c = {}",c);
        c.push_str(" World!");
    }
    println!("b = {}",b);
}

fn main() {
  test_borrow();
}
