// Rust uses a concept of lifetimes to interpret how long a variable
// lives. Note the use of the borrow checker when evaluating lifetimes.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// The following example shows lifetimes in action, the ref of x 
// doesn't carry out of scope and thus r would be pointing 
// at deallocated mem
fn failed_lifetime(){
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);
}

// Generic lifetime params that define relationship
// of lifetime between the diff vars.
// Note we're not changing lifetimes here we're jsut telling
// the borrow checker what to look for
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

// An example of generics, traits and lifetimes:
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    // static lifetimes live until program is done
    let s: &'static str = "I have a static lifetime.";

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }
    // This would fail since result has a lifetime of smallest from string1 and string2
    // println!("The longest string is {}", result);
}

// Lifetime Elision rules: compiler can infer lifetimes within these boundaries
// 
// 1. each parameter that is a reference gets its own lifetime parameter in fn definition
// 2. *if there is 1 input lifetime params* then that lifetime is assigned to all output lifetime parameters
// 3. *if there are multiple input lifetime params, but one of them is &self or &mut self* because this 
//    is a method, the lifetime of self is assigned to all output lifetime parameters
