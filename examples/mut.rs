fn test_mut() {
    let mut x = 10;
    println!("in test_mut #1 x = {}",x);
    x = x + 10; 			// x = 20 
    println!("in test_mut #2 x = {}",x);
    {
      let x = 10;               	// x = 10 
      println!("in test_mut #3 x = {}", x);
    }
    let x = x + 10;			// x = ? 
    println!("in test_mut #4 x = {}",x);
}

fn main() {
  test_mut();
}
