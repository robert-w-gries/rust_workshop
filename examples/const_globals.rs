static GLOBAL_FOO: u8 = 4;
static mut GLOBAL_BAR: u32 = 5;
const CONST:i32 = 10;
fn const_globals() {
    println!("Constant CONST {}", CONST);
    println!("Global FOO = {}", GLOBAL_FOO);
    // Modifying global variables is considered 'unsafe' Rust 
    // Uncomment below line to see error
//    println!("Global BAR = {}",GLOBAL_BAR);
}

fn main() {
  const_globals();
}
