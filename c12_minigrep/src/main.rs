// cargo run searchstring example-filename.txt
use std::env;
use std::process;

use c12_minigrep::Config;


fn main() {
    // .collect requires annotation to know what to flatten to (Vec<String>)
    let args: Vec<String> = env::args().collect();
    
    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // how to exit 
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Dont need unwrap_or_else in this case since
    // we're not planning to use the unwrapped value 
    if let Err(e) = c12_minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

