fn main() {

    // Slice
    let array = [0,1,2,3,4,5];
    let s1 = &array; // slice whole array
    let s2 = &array[..]; // slice whole array
    let s3 = &array[1..4]; // [1,2,3]
    println!("Slices s1, s2, s3 are {:?}, {:?}, {:?}", s1, s2, s3);
    println!("Size of slice s1, s2, s3  are {}, {} , {}", s1.len(), s2.len(), s3.len());
}
