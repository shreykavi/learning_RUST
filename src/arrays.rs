// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run(){
    // array of 5 32 bit ints
    let mut numbers: [i32; 4] = [1,2,3,4];

    // Reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);
   
    // Get array length
    println!("Array Length: {}", numbers.len()); 

    // Arrays are stack allocated
    // NOTE: if not 'use' import at TOF this could be std::mem...
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice)
}
