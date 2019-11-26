use std::mem;

fn test_types() {
    let b: bool = true; // let b = true;
    println!("bool is {}", b);
    let i: i32 = 10;
    //let j:u8 = 257;
    let x: usize = 0;
    let y: isize = 0;
    println!(
        "size of usize/isize is {}/{}",
        mem::size_of_val(&x),
        mem::size_of_val(&y)
    );
    // integer_overflow();

    let arr = [1, 2, 3, 4, 5];
    println!("arr is {:?}", arr);
    // array_overflow();

    // tuple : t(,)
    let t1 = ("Bldg 13", 100);
    let t2: (i32, f64) = (1, 1.5);
    let (x, y) = t2; // x=1, y=1.5 (Destructure the tuple)
    println!("Tuples: {:?}, {:?}, {}, {}", t1, t2, x, y);

    // Vectors
    let mut v1: Vec<u8> = vec![];
    v1.push(10);
    v1.push(20);
    let mut v2 = Vec::new();
    v2.push("hi");
    v2.push("hello");
    println!("Vectors v1 and v2 are {:?} and {:?}", v1, v2);

    // Slice
    let array = [0, 1, 2, 3, 4, 5];
    let s1 = &array; // slice whole array
    let s2 = &array[1..4]; // [1,2,3]
    let s3 = &array[..]; // slice whole array
    println!("Slices s1, s2, s3 are {:?}, {:?}, {:?}", s1, s2, s3);
}

fn main() {
    test_types();
}
