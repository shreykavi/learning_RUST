pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {} for String: {}", 2, "test string");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Shrey", "school", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Shrey",
        activity = "Basketball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    println!("10 + 10 = {}", 10 + 10);
}
