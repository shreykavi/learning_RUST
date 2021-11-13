use std::env;

pub fn run() {
    // run `cargo run hello` and args are passed in 
    // args[0] is always file path
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Shrey";

    println!("Args {:?}", args);
    println!("Command {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    }
}