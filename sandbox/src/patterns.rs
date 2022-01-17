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

pub fn run(){
    conditional_ex();
    while_ex();
    for_ex();
    print_ex();
}