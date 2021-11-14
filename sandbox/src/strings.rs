// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data struct - Use when you need to modify or own

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length:{}", hello.len());

    // Push a char
    hello.push('W');

    // Push a string
    hello.push_str("orld");

    // Get capacity in bytes
    println!("Capacity:{}", hello.capacity());

    // Check empty
    println!("Is Empty:{}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!("{}", hello);

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
