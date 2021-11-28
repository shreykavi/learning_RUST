// Vectors - Resizable arrays

use std::mem;

pub fn run(){
    // Initialize empty vector
    let v: Vec<i32> = Vec::new();

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

    // Notice that .get() returns an optional 
    match numbers.get(2) {
        Some(20) => println!("The third element is {}", numbers[2]),
        None => println!("There is no third element."),
        _ => ()
    }
   
    // Get Vector length
    println!("Vector Length: {}", numbers.len()); 

    // Vectors are stack allocated
    // NOTE: if not 'use' import at TOF this could be std::mem...
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // iterating vectors
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    for i in &numbers {
        println!("Number: {}", i);
    }
    
    for x in numbers.iter_mut() {
        // note the * dereferences x
        *x *= 2;
    }
    for i in &mut numbers {
        *i += 50;
    }

    println!("Numbers Vec: {:?}", numbers);
}
