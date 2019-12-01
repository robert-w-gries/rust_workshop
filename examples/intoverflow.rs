// Runtime panic detected for integer overflow

const INT_MAX:i32 = 2147483647;

fn compare(a:i32) -> bool {

 let a: i32 = INT_MAX;
 if a+1 > a {
   true
 }
 else {
   false 
 }
}

fn main() {
  println!("INT_MAX = {}", INT_MAX);
  println!("(INT_MAX +1) > INT_MAX : {}",compare(INT_MAX));
}
