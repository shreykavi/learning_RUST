// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data struct - Use when you need to modify or own

// Difference between str vs String:
// 
// - An &str points memory at a slice of a String. 
// - If we dont need to own or mutate the text it’s working with, it should take a &str instead of a String

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length:{}", hello.len());

    // Push a char
    hello.push('W');

    // Push a string
    // this does not take ownership of vars passed to it (takes a string slice)
    hello.push_str("orld");

    // String concatenation:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used but s2 

    // The `format` macro works like println but returns String to var 
    let s = format!("{}-{}", s2, s3);

    // This print slices by specified range of bytes
    println!("This is a slice of 0..1 {:?}", &hello[0..1]);

    // A better way of iterate the chars of a string is to use .chars()
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

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

    // Use of slices and strings
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error! clear requires a mut reference but println uses word with immmutable ref -> breaks ownership rules
    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}