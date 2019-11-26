fn main() {
    // Vectors
    let mut v1: Vec<u8> = vec![];
    v1.push(10);
    v1.push(20);
    let mut v2 = Vec::new();
    v2.push("hi");
    v2.push("hello");
    println!("Vectors v1 and v2 are {:?} and {:?}", v1, v2);
}
