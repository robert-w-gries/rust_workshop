#[cfg(feature = "broken")]
fn case1() -> &String {
   let x = String::from("hello"); 
   println!("case 1 {}", x);
   &x 
}

// Below case is not possible with Rust.
// Rust frees the memory automatically when
// the owner goes out of scope and hence
// rust is devoid of such memory freeing errors.
// char *case2() {
// char *x = (char*) malloc(10);
// strcpy(x,"hello");
// free(x);
// printf("\ncase 2 %s", x);
// return x;
// }

fn main() {
  #[cfg(feature = "broken")]
  println!("after case1() {}", case1());
}
