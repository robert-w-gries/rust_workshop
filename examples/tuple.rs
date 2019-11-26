fn main() {
    // tuple : t(,)
    let t1 = ("Bldg 13", 100);
    let t2: (i32, f64) = (1, 1.5);
    let (x, y) = t2; // x=1, y=1.5 (Destructure the tuple)
    println!("Tuples: {:?}, {:?}, {}, {}", t1, t2, x, y);
    println!("Access tuples by position {} {}", t2.0, t2.1);
}
