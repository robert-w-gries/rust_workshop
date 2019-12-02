// Double free is not possible in Rust;
// At any point in time, only one variable can 'own' memory.
// Rust tracks the lifetime of variables at compile time and 
// automatically frees up memory/resource when the owner 
// goes out of scope. 
// Borrowing/Ownership rules prevent memory errors 
// like double-free, use-after-free, etc

#[cfg(feature = "broken")]
fn test_with_refs() {
  let new_owner: &String ;
  {
    let owner = String::from("Hello"); 
    new_owner = &owner;
    println!("new owner is {} ",new_owner);
    println!("owner is {}", new_owner);
  } // owner goes out of scope, "hello" is freed.
  println!("new owner is {} ",new_owner);
}

#[cfg(feature = "broken")]
fn test_without_refs() {
  let owner = String::from("hello"); 
  let new_owner = owner;
  println!("new owner is {} ",new_owner);
  println!("owner is {}", owner);
}

fn main() {
    #[cfg(feature = "broken")]
    test_without_refs();
    #[cfg(feature = "broken")]
    test_with_refs();
}

