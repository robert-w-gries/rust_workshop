
fn main() {
  let mut name ;
  {
    let newname = String:: from ("hello world");
    name = &newname; 
  }
  println!("name : {} ", name); // borrower cannot outlive the owner 
}
