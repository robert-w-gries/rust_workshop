
union TestUnion {
  f1: i32,
  f2: i16
}

fn main() {
  let mut u:TestUnion = TestUnion {f2: 100};
// Compilation error without the unsafe block
// Union reads are unsafe 
  unsafe {
    println!("f2 = {}",u.f2);
  }
  u.f1 = 1000;
  unsafe {
    // initialize as f1 and read as f2
    println!("f2 = {}",u.f2);
  }
  
}
