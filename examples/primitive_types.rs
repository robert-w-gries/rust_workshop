#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

fn array_overflow() {
    let arr = [1,2,3,4];
    for i in 0..5 {
        println!("array[{}] = {}", i, arr[i]);
    }
}

fn integer_overflow() {
   let a:i32=2147483647; // 7fff ffff
    if a+1 > a {
        println!("no overflow");
    }
    else {
        println!("overflow");
    }
}

fn type_error() {
    let a:i32 = 251;
    let b:u32 = 251;
    // uncomment the if-else to see the error
   // if a == b {println!("a==b");}
    //else {println!("a != b");}
}

fn main() {
    let b: bool = true; // let b = true;
    println!("bool is {}", b);
    let i:i32 = 10;
    //let j:u8 = 257; // error
    let x:usize = 0;
    let y:isize = 0;
    println!("size of usize/isize is {}/{}",mem::size_of_val(&x), mem::size_of_val(&y));

    // integer_overflow();

    let arr = [1,2,3,4,5];
    println!("arr is {:?}",arr);
    // array_overflow();
}
