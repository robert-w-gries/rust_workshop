fn print_hello() {
    let str1 = "Hi!"; //  str1:&str
    let mut str2 = String::new(); // Vec<u8>
    str2.push_str("Hello");
    str2.push(' ');
    str2.push_str("World!");
    println!("{},{}", str1, str2);
    // println!("{}",str2[0]); // This is not allowed. Why ?
}

fn main() {
    print_hello();
}
