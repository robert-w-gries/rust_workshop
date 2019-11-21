fn main() {

    // Slice
    let array = [0,1,2,3,4,5];
    let s1 = &array; // slice whole array
    let s2 = &array[1..4]; // [1,2,3]
    let s3 = &array[..]; // slice whole array
    println!("Slices s1, s2, s3 are {:?}, {:?}, {:?}", s1, s2, s3);

}
