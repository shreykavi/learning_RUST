// Vectors - Resizable arrays

use std::mem;

pub fn run(){
    // Vector of 4 32 bit ints
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Reassign value
    numbers[2] = 20;

    // Add onto vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);
   
    // Get Vector length
    println!("Vector Length: {}", numbers.len()); 

    // Vectors are stack allocated
    // NOTE: if not 'use' import at TOF this could be std::mem...
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    
    for x in numbers.iter_mut() {
        // note the * dereferences x
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
