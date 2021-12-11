// Note that running `RUST_BACKTRACE=1 cargo run` will
// give us backtracing of error stack

use std::fs::File;
use std::io::{self, Read, ErrorKind};


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        // This propagates err up rather than panicking
        // its the callers responsibility to either panic
        // or handle correctly.
        Err(e) => return Err(e), 
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// we’re only allowed to use the ? operator in a function that returns Result 
// or Option or another type that implements std::ops::Try
fn read_username_from_file_shorthand() -> Result<String, io::Error> {
    // Uses '?' operator to propagate Err or Ok up
    // note: ? auto converts error types into return type! 
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    // chained ?
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

pub fn run(){
    // panic!("Crash and burn mfker")

    // Rather than buffer overread this will just panic
    // It is safer not to give access to that memory 
    // (like would be in C)
    // 
    // let v = vec![1, 2, 3];
    // v[99];

    let f: Result<File, std::io::Error> = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // Handles different kinds of errors
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Another impl using closures instead of match
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // .unwrap() or .expect() can also be used to safe check

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}