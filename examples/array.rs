#![allow(dead_code)]
#![allow(unused_variables)]

fn array_overflow() {

    // Define arr of type i32 containing 4 elements, initialized to 0.
    let mut arr:[i32;4] =  [0;4];


    arr = [1, 2, 8, 4];

    for i in 0..8 {
        println!("array[{}] = {}", i, arr[i]);
    }

    let mut sum = 0;

    // Iterate through all elements of the array.
    // This is an efficient way to iterate the array 
    // where an overflow can be eliminated
    for i in &arr {
 	sum += i;
    }
    println!("sum of array {:?} is {}", arr, sum);
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    // {:?} is used to print the structure in debug format
    println!("arr is {:?}", arr);

    #[cfg(feature = "broken")]
    array_overflow();
}
