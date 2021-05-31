// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Shrey";

    // Mutable var 
    let mut age = 24;
        age = 25; // Not actually lol

    println!("My name is {} and I am {}", name, age);

    // Define constant (Note the type and capitalized name)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Shrey", 24);
    println!("{} is {}", my_name, my_age);
}