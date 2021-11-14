// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
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

    let legal_drinker = "Yes";

    {// This defines an inner scope
        // This is a shadow that reassigns the val of the var
        // Shadows are useful as we can redefine type of var aswell
        let legal_drinker = true;
        println!("{} is a legal_drinker {}", name, legal_drinker);
        
        // Note that at end of scope Rust will free up memory for vars in this scope
        let mut some_var = String::from("Hello.");
    }

    // Notice the value of the var changes back once out of scope
    println!("{} is a legal_drinker {}", name, legal_drinker);
}
