// This is how we define module trees
// crate
//  ├── front_of_house
//  │    ├── hosting
//  │    │   ├── add_to_waitlist
//  │    │   └── seat_at_table
//  │    └── serving
//  │        ├── take_order
//  │        ├── serve_order
//  │        └── take_payment
//  └── back_of_house

// Load from another file
// Note the pub on import here does a re-export
pub mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super is like `../` in relative paths 
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, //not pub so user can't change
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer { // pub enum makes all variants public
        Soup,
        Salad,
    }
}


// idiomatic to bring module NOT just fn into scope
use crate::front_of_house::hosting; // absolute
// use self::front_of_house::hosting; // relative

// NOTE: with structs, enums, and other items
// we bring into scope by specifying full path
use std::collections::HashMap;

// We can also use as to rename imported object
use std::collections::HashMap as hashiemapie;

// Importing multiple from a module
use std::io::{self, Write};

// Importing all from a module
use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Use path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}
