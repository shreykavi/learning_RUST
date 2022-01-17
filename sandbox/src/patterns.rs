// Irrefutable: Pattern that will never fail to match (ex. assigning a var)
// Refutable: Patterns that will fail to match in some cases
// this concept is only important when sene

fn conditional_ex(){
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "24".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // Notice the previous line introduces a shadow var for the Ok variant of age
        // but the variant isnt available until the next scope is open
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_ex(){
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Loops until some doesnt return any value on pop
    // this is matching for a Some pattern
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_ex(){
    let v = vec!['a', 'b', 'c'];

    // This is matching for an enumerated value pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn print_ex() {
    // In this ex the point tuple is pattern matched to the functions params
    let point = (3, 5);
    print_coordinates(&point);
}

fn refutable_ex(){
    // The following fails since assignments cannot be made with None in Rust
    // A value must exist and so we need an irrefutable pattern
    // 
    // let Some(x) = some_option_value {}; // where some_option_value can be 5 for ex. 

    // By adding `if` we make it conditionally use the value if returned
    // But if we use an irrefutable pattern it would error
    // 
    // if let Some(x) = 5 {
    //     println!("{}", x);
    // }
}

pub fn run(){
    conditional_ex();
    while_ex();
    for_ex();
    print_ex();
    refutable_ex();
}