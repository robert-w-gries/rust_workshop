#[cfg(feature = "broken")]
fn test() {
  let a:i8 = 251;
  let b:u8 = 251;
  if a ==  b {
    println!("Same");
  }
  else {
    println!("Not same");
  }
  println!("{} {}", a, b);
}

fn main() {
#[cfg(feature = "broken")]
  test();
}
