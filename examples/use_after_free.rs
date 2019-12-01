fn main() {
 let mut x2;
 {
   let temp = String::from("Hello World");
   x2 = &temp;
   println!("\nx2 use before drop {}", x2);
 }
 println!("\nx2 use after free {}", x2); // use after free
}
